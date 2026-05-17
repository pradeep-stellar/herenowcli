use std::fs;

use anyhow::{anyhow, Context, Result};
use reqwest::header::{HeaderMap, HeaderValue, IF_MATCH};
use serde_json::Value;

use crate::{
    api::models::{
        DriveCreateRequest, DriveFinalizeRequest, DriveMoveRequest, DriveTokenCreateRequest,
        DriveUploadCreateRequest,
    },
    cli::{DrivesCommand, DrivesSubcommand},
    confirm,
    files::hash_file,
    http::ApiClient,
    output,
};

pub async fn run(client: &ApiClient, cmd: DrivesCommand, json_output: bool) -> Result<()> {
    match cmd.command {
        DrivesSubcommand::List => {
            let response: Value = client.get("/api/v1/drives").await?;
            output::json(&response)
        }
        DrivesSubcommand::Default => {
            let response: Value = client.get("/api/v1/drives/default").await?;
            output::json(&response)
        }
        DrivesSubcommand::Create { name } => {
            let response: Value = client
                .post("/api/v1/drives", &DriveCreateRequest { name })
                .await?;
            output::json(&response)
        }
        DrivesSubcommand::Get { drive_id } => {
            let response: Value = client.get(&format!("/api/v1/drives/{drive_id}")).await?;
            output::json(&response)
        }
        DrivesSubcommand::Files { drive_id, prefix } => {
            let path = match prefix {
                Some(prefix) => format!(
                    "/api/v1/drives/{drive_id}/files?prefix={}",
                    urlencoding::encode(&prefix)
                ),
                None => format!("/api/v1/drives/{drive_id}/files"),
            };
            let response: Value = client.get(&path).await?;
            output::json(&response)
        }
        DrivesSubcommand::Cat { drive_id, path } => {
            let response = client
                .get_bytes(&format!(
                    "/api/v1/drives/{drive_id}/files/{}",
                    encode_path(&path)
                ))
                .await?;
            print!("{}", String::from_utf8_lossy(&response));
            Ok(())
        }
        DrivesSubcommand::Put {
            drive_id,
            path,
            from,
            if_match,
            if_none_match,
        } => {
            let bytes = fs::read(&from)
                .with_context(|| format!("failed to read source file {}", from.display()))?;
            let size = bytes.len() as u64;
            let content_type = mime_guess::from_path(&from)
                .first_or_octet_stream()
                .essence_str()
                .to_string();
            let hash = hash_file(&from)?;
            let upload: Value = client
                .post(
                    &format!("/api/v1/drives/{drive_id}/files/uploads"),
                    &DriveUploadCreateRequest {
                        path: path.clone(),
                        size,
                        content_type: Some(content_type),
                        sha256: Some(hash),
                        if_match,
                        if_none_match,
                    },
                )
                .await?;
            let url = upload
                .get("url")
                .and_then(Value::as_str)
                .ok_or_else(|| anyhow!("Drive upload response did not include url"))?
                .to_string();
            let headers = upload
                .get("headers")
                .cloned()
                .map(serde_json::from_value)
                .transpose()?
                .unwrap_or_default();
            let upload_id = upload
                .get("uploadId")
                .and_then(Value::as_str)
                .ok_or_else(|| anyhow!("Drive upload response did not include uploadId"))?
                .to_string();
            client.put_upload(&url, bytes, &headers).await?;
            let response: Value = client
                .post(
                    &format!("/api/v1/drives/{drive_id}/files/finalize"),
                    &DriveFinalizeRequest {
                        upload_id: Some(upload_id),
                        path: Some(path),
                    },
                )
                .await?;
            output::value_or_json(json_output, &response, || {
                println!("Drive file uploaded.");
                Ok(())
            })
        }
        DrivesSubcommand::DeleteFile {
            drive_id,
            path,
            if_match,
            yes,
        } => {
            confirm::confirm_or_bail(
                &format!("Delete Drive file '{path}' from '{drive_id}' permanently?"),
                yes,
            )?;
            let mut headers = HeaderMap::new();
            headers.insert(IF_MATCH, HeaderValue::from_str(&if_match)?);
            let response: Value = client
                .delete_with_headers(
                    &format!("/api/v1/drives/{drive_id}/files/{}", encode_path(&path)),
                    headers,
                )
                .await?;
            output::value_or_json(json_output, &response, || {
                println!("Drive file deleted.");
                Ok(())
            })
        }
        DrivesSubcommand::Move {
            drive_id,
            from,
            to,
            if_match,
            overwrite_if_match,
        } => {
            let response: Value = client
                .post(
                    &format!("/api/v1/drives/{drive_id}/files/move"),
                    &DriveMoveRequest {
                        from,
                        to,
                        if_match,
                        overwrite_if_match,
                    },
                )
                .await?;
            output::json(&response)
        }
        DrivesSubcommand::Tokens { drive_id } => {
            let response: Value = client
                .get(&format!("/api/v1/drives/{drive_id}/tokens"))
                .await?;
            output::json(&response)
        }
        DrivesSubcommand::Share {
            drive_id,
            perms,
            prefix,
            ttl,
            label,
            manage_tokens,
        } => {
            let response: Value = client
                .post(
                    &format!("/api/v1/drives/{drive_id}/tokens"),
                    &DriveTokenCreateRequest {
                        perms,
                        path_prefix: prefix,
                        ttl,
                        label,
                        manage_tokens: manage_tokens.then_some(true),
                    },
                )
                .await?;
            output::json(&response)
        }
    }
}

fn encode_path(path: &str) -> String {
    path.split('/')
        .map(|segment| urlencoding::encode(segment).into_owned())
        .collect::<Vec<_>>()
        .join("/")
}
