use std::{collections::BTreeMap, fs, io, path::PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    #[serde(default)]
    pub publishes: BTreeMap<String, PublishState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishState {
    #[serde(rename = "siteUrl")]
    pub site_url: String,
    #[serde(rename = "claimToken", skip_serializing_if = "Option::is_none")]
    pub claim_token: Option<String>,
    #[serde(rename = "claimUrl", skip_serializing_if = "Option::is_none")]
    pub claim_url: Option<String>,
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

pub fn load() -> Result<State> {
    let path = path();
    match fs::read_to_string(&path) {
        Ok(data) => serde_json::from_str(&data)
            .with_context(|| format!("failed to parse {}", path.display())),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(State::default()),
        Err(err) => Err(err).with_context(|| format!("failed to read {}", path.display())),
    }
}

pub fn save(state: &State) -> Result<()> {
    let path = path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    let data = serde_json::to_string_pretty(state)?;
    fs::write(&path, data).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(())
}

pub fn remember_publish(
    slug: &str,
    site_url: String,
    claim_token: Option<String>,
    claim_url: Option<String>,
    expires_at: Option<String>,
) -> Result<()> {
    let mut state = load()?;
    state.publishes.insert(
        slug.to_string(),
        PublishState {
            site_url,
            claim_token,
            claim_url,
            expires_at,
        },
    );
    save(&state)
}

pub fn claim_token(slug: &str) -> Result<Option<String>> {
    Ok(load()?
        .publishes
        .get(slug)
        .and_then(|publish| publish.claim_token.clone()))
}

fn path() -> PathBuf {
    PathBuf::from(".herenow").join("state.json")
}
