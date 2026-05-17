use std::{env, fs, io, path::PathBuf};

use anyhow::{Context, Result};

use crate::cli::Cli;

#[derive(Clone, Debug)]
pub struct Config {
    pub base_url: String,
    pub api_key: Option<String>,
    pub client_name: String,
}

impl Config {
    pub fn from_cli(cli: &Cli) -> Result<Self> {
        Ok(Self {
            base_url: cli.base_url.trim_end_matches('/').to_string(),
            api_key: credential(cli.api_key.clone())?,
            client_name: cli.client.clone(),
        })
    }
}

pub fn credentials_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("could not find home directory")?;
    Ok(home.join(".herenow").join("credentials"))
}

pub fn save_api_key(api_key: &str) -> Result<()> {
    let path = credentials_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(&path, format!("{api_key}\n"))
        .with_context(|| format!("failed to write {}", path.display()))?;
    set_private_permissions(&path)?;
    Ok(())
}

fn credential(cli_key: Option<String>) -> Result<Option<String>> {
    if let Some(key) = clean_key(cli_key) {
        return Ok(Some(key));
    }
    if let Some(key) = clean_key(env::var("HERENOW_API_KEY").ok()) {
        return Ok(Some(key));
    }
    let path = credentials_path()?;
    match fs::read_to_string(&path) {
        Ok(value) => Ok(clean_key(Some(value))),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err).with_context(|| format!("failed to read {}", path.display())),
    }
}

fn clean_key(value: Option<String>) -> Option<String> {
    value
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

#[cfg(unix)]
fn set_private_permissions(path: &PathBuf) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path)?.permissions();
    permissions.set_mode(0o600);
    fs::set_permissions(path, permissions)?;
    Ok(())
}

#[cfg(not(unix))]
fn set_private_permissions(_path: &PathBuf) -> Result<()> {
    Ok(())
}
