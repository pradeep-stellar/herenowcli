use anyhow::{Context, Result};
use reqwest::{header::HeaderMap, Method};
use serde::{de::DeserializeOwned, Serialize};

use crate::{config::Config, error::ApiError};

#[derive(Clone)]
pub struct ApiClient {
    inner: reqwest::Client,
    config: Config,
}

impl ApiClient {
    pub fn new(config: Config) -> Result<Self> {
        let inner = reqwest::Client::builder()
            .user_agent(&config.client_name)
            .build()
            .context("failed to build HTTP client")?;
        Ok(Self { inner, config })
    }

    pub fn has_auth(&self) -> bool {
        self.config.api_key.is_some()
    }

    pub fn base_url(&self) -> &str {
        &self.config.base_url
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.request_json(Method::GET, path, Option::<&()>::None)
            .await
    }

    pub async fn get_bytes(&self, path: &str) -> Result<bytes::Bytes> {
        let url = self.url(path);
        let request = self.apply_headers(self.inner.request(Method::GET, url));
        let response = request.send().await.context("request failed")?;
        let status = response.status();
        let bytes = response
            .bytes()
            .await
            .context("failed to read response body")?;
        if status.is_success() {
            Ok(bytes)
        } else {
            let api_error = serde_json::from_slice::<ApiError>(&bytes).unwrap_or_else(|_| {
                ApiError::fallback(status, String::from_utf8_lossy(&bytes).to_string())
            });
            Err(api_error).context("API request failed")
        }
    }

    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.request_json(Method::DELETE, path, Option::<&()>::None)
            .await
    }

    pub async fn post<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        self.request_json(Method::POST, path, Some(body)).await
    }

    pub async fn put<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        self.request_json(Method::PUT, path, Some(body)).await
    }

    pub async fn patch<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        self.request_json(Method::PATCH, path, Some(body)).await
    }

    pub async fn delete_with_headers<T: DeserializeOwned>(
        &self,
        path: &str,
        headers: HeaderMap,
    ) -> Result<T> {
        let url = self.url(path);
        let mut request = self.inner.request(Method::DELETE, url);
        request = self.apply_headers(request).headers(headers);
        let response = request.send().await.context("request failed")?;
        decode(response).await
    }

    pub async fn put_upload(
        &self,
        url: &str,
        bytes: Vec<u8>,
        headers: &std::collections::BTreeMap<String, String>,
    ) -> Result<()> {
        let mut request = self.inner.request(Method::PUT, url).body(bytes);
        for (name, value) in headers {
            request = request.header(name.as_str(), value);
        }
        let response = request.send().await.context("upload request failed")?;
        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(ApiError::fallback(status, body)).context("upload failed")
        }
    }

    async fn request_json<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T> {
        let url = self.url(path);
        let mut request = self.inner.request(method, url);
        request = self.apply_headers(request);
        if let Some(body) = body {
            request = request.json(body);
        }
        let response = request.send().await.context("request failed")?;
        decode(response).await
    }

    fn apply_headers(&self, mut request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        request = request.header("X-HereNow-Client", &self.config.client_name);
        if let Some(api_key) = &self.config.api_key {
            request = request.bearer_auth(api_key);
        }
        request
    }

    fn url(&self, path: &str) -> String {
        if path.starts_with("http://") || path.starts_with("https://") {
            path.to_string()
        } else {
            format!("{}{}", self.config.base_url, path)
        }
    }
}

async fn decode<T: DeserializeOwned>(response: reqwest::Response) -> Result<T> {
    let status = response.status();
    let bytes = response
        .bytes()
        .await
        .context("failed to read response body")?;
    if status.is_success() {
        serde_json::from_slice(&bytes).context("failed to decode response JSON")
    } else {
        let api_error = serde_json::from_slice::<ApiError>(&bytes).unwrap_or_else(|_| {
            ApiError::fallback(status, String::from_utf8_lossy(&bytes).to_string())
        });
        Err(api_error).context("API request failed")
    }
}
