use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAuthRequestCodeRequest {
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAuthRequestCodeResponse {
    pub ok: Option<bool>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAuthVerifyCodeRequest {
    pub email: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAuthVerifyCodeResponse {
    #[serde(rename = "apiKey")]
    pub api_key: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewerMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ogImagePath", skip_serializing_if = "Option::is_none")]
    pub og_image_path: Option<String>,
}

impl ViewerMetadata {
    pub fn is_empty(&self) -> bool {
        self.title.is_none() && self.description.is_none() && self.og_image_path.is_none()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishFile {
    pub path: String,
    pub size: u64,
    #[serde(rename = "contentType")]
    pub content_type: String,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishCreateRequest {
    pub files: Vec<PublishFile>,
    #[serde(rename = "ttlSeconds", skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerMetadata>,
    #[serde(rename = "claimToken", skip_serializing_if = "Option::is_none")]
    pub claim_token: Option<String>,
    #[serde(rename = "spaMode", skip_serializing_if = "Option::is_none")]
    pub spa_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forkable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishCreateResponse {
    pub slug: String,
    #[serde(rename = "siteUrl")]
    pub site_url: String,
    pub upload: PublishUpload,
    #[serde(rename = "claimToken")]
    pub claim_token: Option<String>,
    #[serde(rename = "claimUrl")]
    pub claim_url: Option<String>,
    #[serde(rename = "expiresAt")]
    pub expires_at: Option<String>,
    pub anonymous: Option<bool>,
    pub warning: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishUpload {
    #[serde(rename = "versionId")]
    pub version_id: String,
    pub uploads: Vec<UploadTarget>,
    #[serde(default)]
    pub skipped: Vec<String>,
    #[serde(rename = "finalizeUrl")]
    pub finalize_url: String,
    #[serde(rename = "expiresInSeconds")]
    pub expires_in_seconds: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadTarget {
    pub path: String,
    pub method: Option<String>,
    pub url: String,
    #[serde(default)]
    pub headers: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalizePublishRequest {
    #[serde(rename = "versionId")]
    pub version_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalizePublishResponse {
    pub success: bool,
    pub slug: String,
    #[serde(rename = "siteUrl")]
    pub site_url: String,
    #[serde(rename = "previousVersionId")]
    pub previous_version_id: Option<String>,
    #[serde(rename = "currentVersionId")]
    pub current_version_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimSiteRequest {
    #[serde(rename = "claimToken")]
    pub claim_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishFromDriveRequest {
    #[serde(rename = "driveId")]
    pub drive_id: String,
    #[serde(rename = "versionId", skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishMetadataPatchRequest {
    #[serde(rename = "ttlSeconds", skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer: Option<ViewerMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<Option<String>>,
    #[serde(rename = "spaMode", skip_serializing_if = "Option::is_none")]
    pub spa_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forkable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishListResponse {
    pub publishes: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveCreateRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveUploadCreateRequest {
    pub path: String,
    pub size: u64,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    #[serde(rename = "sha256", skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(rename = "ifMatch", skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
    #[serde(rename = "ifNoneMatch", skip_serializing_if = "Option::is_none")]
    pub if_none_match: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveFinalizeRequest {
    #[serde(rename = "uploadId", skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveMoveRequest {
    pub from: String,
    pub to: String,
    #[serde(rename = "ifMatch")]
    pub if_match: String,
    #[serde(rename = "overwriteIfMatch", skip_serializing_if = "Option::is_none")]
    pub overwrite_if_match: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveTokenCreateRequest {
    pub perms: String,
    #[serde(rename = "pathPrefix", skip_serializing_if = "Option::is_none")]
    pub path_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "manageTokens", skip_serializing_if = "Option::is_none")]
    pub manage_tokens: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDomainRequest {
    pub domain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandleRequest {
    pub handle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkRequest {
    pub location: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableSetRequest {
    pub value: String,
    #[serde(
        rename = "allowedUpstreams",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub allowed_upstreams: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletPatchRequest {
    pub address: Option<String>,
}
