use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiError {
    pub error: Option<String>,
    pub code: Option<String>,
    pub message: Option<String>,
    pub retry_after: Option<u64>,
    pub docs_url: Option<String>,
}

impl ApiError {
    pub fn fallback(status: reqwest::StatusCode, body: String) -> Self {
        Self {
            error: Some(format!("HTTP {status}")),
            code: None,
            message: if body.trim().is_empty() {
                None
            } else {
                Some(body)
            },
            retry_after: None,
            docs_url: None,
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let main = self
            .message
            .as_deref()
            .or(self.error.as_deref())
            .unwrap_or("API request failed");
        match (&self.code, self.retry_after.as_ref(), &self.docs_url) {
            (Some(code), Some(retry), Some(docs)) => {
                write!(f, "{main} ({code}, retry after {retry}s, docs: {docs})")
            }
            (Some(code), Some(retry), None) => write!(f, "{main} ({code}, retry after {retry}s)"),
            (Some(code), None, Some(docs)) => write!(f, "{main} ({code}, docs: {docs})"),
            (Some(code), None, None) => write!(f, "{main} ({code})"),
            _ => write!(f, "{main}"),
        }
    }
}

impl std::error::Error for ApiError {}
