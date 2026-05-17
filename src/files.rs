use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

use crate::api::models::PublishFile;

#[derive(Debug, Clone)]
pub struct LocalFile {
    pub absolute: PathBuf,
    pub publish: PublishFile,
}

pub fn collect_publish_files(input: &Path) -> Result<Vec<LocalFile>> {
    if input.is_file() {
        let file_name = input
            .file_name()
            .and_then(|name| name.to_str())
            .context("input file must have a UTF-8 file name")?;
        return Ok(vec![describe_file(input, file_name)?]);
    }

    if !input.is_dir() {
        bail!("{} is not a file or directory", input.display());
    }

    let mut files = Vec::new();
    for entry in WalkDir::new(input).follow_links(false) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let relative = entry
            .path()
            .strip_prefix(input)
            .context("failed to build relative publish path")?;
        if relative
            .components()
            .any(|part| part.as_os_str().to_str() == Some(".herenow"))
        {
            continue;
        }
        let relative = normalize_relative_path(relative)?;
        files.push(describe_file(entry.path(), &relative)?);
    }
    files.sort_by(|a, b| a.publish.path.cmp(&b.publish.path));

    if files.is_empty() {
        bail!("{} does not contain any publishable files", input.display());
    }
    Ok(files)
}

pub fn file_map(files: &[LocalFile]) -> BTreeMap<String, PathBuf> {
    files
        .iter()
        .map(|file| (file.publish.path.clone(), file.absolute.clone()))
        .collect()
}

pub fn hash_file(path: &Path) -> Result<String> {
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(format!("{:x}", hasher.finalize()))
}

fn describe_file(path: &Path, publish_path: &str) -> Result<LocalFile> {
    let metadata =
        fs::metadata(path).with_context(|| format!("failed to stat {}", path.display()))?;
    let content_type = mime_guess::from_path(path)
        .first_or_octet_stream()
        .essence_str()
        .to_string();
    Ok(LocalFile {
        absolute: path.to_path_buf(),
        publish: PublishFile {
            path: publish_path.to_string(),
            size: metadata.len(),
            content_type,
            hash: hash_file(path)?,
        },
    })
}

fn normalize_relative_path(path: &Path) -> Result<String> {
    let parts: Option<Vec<_>> = path
        .components()
        .map(|component| component.as_os_str().to_str().map(ToOwned::to_owned))
        .collect();
    let path = parts.context("publish paths must be UTF-8")?.join("/");
    if path.is_empty() || path.starts_with('/') || path.contains("..") {
        bail!("invalid publish path {path:?}");
    }
    Ok(path)
}
