use std::{fs, path::PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use futures::{stream, StreamExt};
use serde_json::{json, Value};

use crate::{
    api::models::{
        ClaimSiteRequest, FinalizePublishRequest, FinalizePublishResponse, PublishCreateRequest,
        PublishCreateResponse, PublishFromDriveRequest, PublishListResponse,
        PublishMetadataPatchRequest, ViewerMetadata,
    },
    cli::{PublishCommand, SitesCommand, SitesSubcommand},
    confirm,
    files::{collect_publish_files, file_map},
    http::ApiClient,
    output, state,
};

pub async fn run_publish(client: &ApiClient, cmd: PublishCommand, json_output: bool) -> Result<()> {
    let files = collect_publish_files(&cmd.path)?;
    let local_files = file_map(&files);
    let viewer = viewer(cmd.title, cmd.description, cmd.og_image_path);
    let claim_token = cmd.claim_token.or_else(|| {
        cmd.slug
            .as_ref()
            .and_then(|slug| state::claim_token(slug).ok().flatten())
    });
    let request = PublishCreateRequest {
        files: files.into_iter().map(|file| file.publish).collect(),
        ttl_seconds: cmd.ttl,
        viewer,
        claim_token,
        spa_mode: cmd.spa.then_some(true),
        forkable: cmd.forkable.then_some(true),
    };

    let response: PublishCreateResponse = if let Some(slug) = &cmd.slug {
        client
            .put(&format!("/api/v1/publish/{slug}"), &request)
            .await?
    } else {
        client.post("/api/v1/publish", &request).await?
    };

    upload_pending_files(client, &response, local_files).await?;

    let finalize: FinalizePublishResponse = client
        .post(
            finalize_path(client, &response)?,
            &FinalizePublishRequest {
                version_id: response.upload.version_id.clone(),
            },
        )
        .await?;

    if response.anonymous.unwrap_or(false) || response.claim_token.is_some() {
        state::remember_publish(
            &response.slug,
            response.site_url.clone(),
            response.claim_token.clone(),
            response.claim_url.clone(),
            response.expires_at.clone(),
        )?;
    }

    if json_output {
        output::json(&json!({
            "publish": response,
            "finalize": finalize,
        }))
    } else {
        println!("{}", finalize.site_url);
        if response.anonymous.unwrap_or(false) {
            if let Some(expires_at) = response.expires_at {
                println!("Anonymous site expires at {expires_at}.");
            } else {
                println!("Anonymous site expires in 24 hours.");
            }
            if let Some(claim_url) = response.claim_url {
                println!("Claim URL: {claim_url}");
            }
        } else if client.has_auth() {
            println!("Authenticated site saved to your account.");
        }
        if !response.upload.skipped.is_empty() {
            println!(
                "Skipped {} unchanged file(s).",
                response.upload.skipped.len()
            );
        }
        Ok(())
    }
}

pub async fn run_sites(client: &ApiClient, cmd: SitesCommand, json_output: bool) -> Result<()> {
    match cmd.command {
        SitesSubcommand::List => {
            let response: PublishListResponse = client.get("/api/v1/publishes").await?;
            output::value_or_json(json_output, &response, || {
                for site in &response.publishes {
                    let slug = site.get("slug").and_then(Value::as_str).unwrap_or("-");
                    let url = site.get("siteUrl").and_then(Value::as_str).unwrap_or("-");
                    let status = site.get("status").and_then(Value::as_str).unwrap_or("-");
                    println!("{slug}\t{status}\t{url}");
                }
                Ok(())
            })
        }
        SitesSubcommand::Get { slug } => {
            let response: Value = client.get(&format!("/api/v1/publish/{slug}")).await?;
            output::json(&response)
        }
        SitesSubcommand::Delete { slug, yes } => {
            confirm::confirm_or_bail(&format!("Delete site '{slug}' permanently?"), yes)?;
            let response: Value = client.delete(&format!("/api/v1/publish/{slug}")).await?;
            output::value_or_json(json_output, &response, || {
                println!("Deleted {slug}.");
                Ok(())
            })
        }
        SitesSubcommand::Claim { slug, claim_token } => {
            let claim_token = claim_token
                .or_else(|| state::claim_token(&slug).ok().flatten())
                .ok_or_else(|| anyhow!("claim token required; pass --claim-token"))?;
            let response: Value = client
                .post(
                    &format!("/api/v1/publish/{slug}/claim"),
                    &ClaimSiteRequest { claim_token },
                )
                .await?;
            output::json_or_done(json_output, &response, "Site claimed.")
        }
        SitesSubcommand::Metadata {
            slug,
            title,
            description,
            og_image_path,
            ttl,
            password,
            remove_password,
            spa,
            forkable,
        } => {
            if password.is_some() && remove_password {
                bail!("use either --password or --remove-password, not both");
            }
            let request = PublishMetadataPatchRequest {
                ttl_seconds: ttl,
                viewer: viewer(title, description, og_image_path),
                password: if remove_password {
                    Some(None)
                } else {
                    password.map(Some)
                },
                spa_mode: spa,
                forkable,
            };
            let response: Value = client
                .patch(&format!("/api/v1/publish/{slug}/metadata"), &request)
                .await?;
            output::json_or_done(json_output, &response, "Metadata updated.")
        }
        SitesSubcommand::FromDrive {
            drive_id,
            version,
            slug,
        } => {
            let response: Value = client
                .post(
                    "/api/v1/publish/from-drive",
                    &PublishFromDriveRequest {
                        drive_id,
                        version_id: version,
                        slug,
                    },
                )
                .await?;
            output::value_or_json(json_output, &response, || {
                if let Some(url) = response.get("siteUrl").and_then(Value::as_str) {
                    println!("{url}");
                } else {
                    println!("Published from Drive.");
                }
                Ok(())
            })
        }
    }
}

async fn upload_pending_files(
    client: &ApiClient,
    response: &PublishCreateResponse,
    local_files: std::collections::BTreeMap<String, PathBuf>,
) -> Result<()> {
    let uploads = response.upload.uploads.clone();
    stream::iter(uploads)
        .map(|upload| {
            let client = client.clone();
            let local_files = local_files.clone();
            async move {
                let path = local_files
                    .get(&upload.path)
                    .ok_or_else(|| anyhow!("missing local file for upload {}", upload.path))?;
                let bytes = fs::read(path)
                    .with_context(|| format!("failed to read upload file {}", path.display()))?;
                client.put_upload(&upload.url, bytes, &upload.headers).await
            }
        })
        .buffer_unordered(8)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>()?;
    Ok(())
}

fn finalize_path<'a>(client: &ApiClient, response: &'a PublishCreateResponse) -> Result<&'a str> {
    Ok(response
        .upload
        .finalize_url
        .strip_prefix(client.base_url())
        .unwrap_or(response.upload.finalize_url.as_str()))
}

fn viewer(
    title: Option<String>,
    description: Option<String>,
    og_image_path: Option<String>,
) -> Option<ViewerMetadata> {
    let viewer = ViewerMetadata {
        title,
        description,
        og_image_path,
    };
    (!viewer.is_empty()).then_some(viewer)
}
