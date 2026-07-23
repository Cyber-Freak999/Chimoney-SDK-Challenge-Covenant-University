use reqwest_middleware::ClientWithMiddleware;

use crate::error::{ChimoneyError, Result};
use crate::middleware::{build_client, DEFAULT_MAX_RETRIES, DEFAULT_TIMEOUT_SECS};

/// Chimoney API client.
///
/// # Example
///
/// ```rust
/// use chimoney_rust_sdk::ChimoneyClient;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = ChimoneyClient::new("your_api_key")?;
/// # Ok(())
/// # }
/// ```
pub struct ChimoneyClient {
    client: ClientWithMiddleware,
    api_key: String,
    base_url: String,
}

impl ChimoneyClient {
    /// Create a new ChimoneyClient with default settings.
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::builder(api_key).build()
    }

    /// Create a new ChimoneyClient with sandbox URL.
    pub fn new_sandbox(api_key: impl Into<String>) -> Result<Self> {
        Self::builder(api_key)
            .base_url("https://api-v2-sandbox.chimoney.io")
            .build()
    }

    /// Get a builder for configuring the client.
    pub fn builder(api_key: impl Into<String>) -> ChimoneyClientBuilder {
        ChimoneyClientBuilder {
            api_key: api_key.into(),
            base_url: "https://api.chimoney.io".to_string(),
            max_retries: DEFAULT_MAX_RETRIES,
            timeout_secs: DEFAULT_TIMEOUT_SECS,
        }
    }

    /// Get a reference to the HTTP client.
    pub fn http_client(&self) -> &ClientWithMiddleware {
        &self.client
    }

    /// Get the base URL.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Make a GET request.
    async fn get(&self, path: &str, query: Option<&str>) -> Result<String> {
        let mut url = format!("{}{}", self.base_url, path);
        if let Some(params) = query {
            url.push('?');
            url.push_str(params);
        }

        let response = self
            .client
            .get(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("X-API-KEY", &self.api_key)
            .send()
            .await
            .map_err(ChimoneyError::MiddlewareError)?;

        self.handle_response(response).await
    }

    /// Make a POST request.
    async fn post(&self, path: &str, body: &str, query: Option<&str>) -> Result<String> {
        let mut url = format!("{}{}", self.base_url, path);
        if let Some(params) = query {
            url.push('?');
            url.push_str(params);
        }

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("X-API-KEY", &self.api_key)
            .body(body.to_string())
            .send()
            .await
            .map_err(ChimoneyError::MiddlewareError)?;

        self.handle_response(response).await
    }

    /// Make a DELETE request.
    async fn delete(&self, path: &str, query: Option<&str>) -> Result<String> {
        let mut url = format!("{}{}", self.base_url, path);
        if let Some(params) = query {
            url.push('?');
            url.push_str(params);
        }

        let response = self
            .client
            .delete(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("X-API-KEY", &self.api_key)
            .send()
            .await
            .map_err(ChimoneyError::MiddlewareError)?;

        self.handle_response(response).await
    }

    /// Handle API response.
    async fn handle_response(&self, response: reqwest::Response) -> Result<String> {
        let status = response.status();
        let text = response
            .text()
            .await
            .map_err(ChimoneyError::RequestFailed)?;

        if status.is_success() {
            Ok(text)
        } else {
            let json: serde_json::Value = serde_json::from_str(&text).unwrap_or_else(|_| {
                serde_json::json!({
                    "code": status.as_u16(),
                    "message": text
                })
            });

            let message = json["message"]
                .as_str()
                .unwrap_or("Unknown error")
                .to_string();

            if status.as_u16() == 429 {
                let retry_after = json["retry_after"].as_u64().unwrap_or(60);
                return Err(ChimoneyError::RateLimited { retry_after });
            }

            Err(ChimoneyError::ApiError {
                status: status.as_u16(),
                message,
            })
        }
    }
}

/// Builder for configuring `ChimoneyClient`.
pub struct ChimoneyClientBuilder {
    api_key: String,
    base_url: String,
    max_retries: u32,
    timeout_secs: u64,
}

impl ChimoneyClientBuilder {
    /// Set the base URL.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Set the maximum number of retries.
    pub fn max_retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }

    /// Set the request timeout in seconds.
    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// Build the client.
    pub fn build(self) -> Result<ChimoneyClient> {
        if self.api_key.is_empty() {
            return Err(ChimoneyError::ApiKeyEmpty);
        }

        let client = build_client(self.max_retries, self.timeout_secs)?;

        Ok(ChimoneyClient {
            client,
            api_key: self.api_key,
            base_url: self.base_url,
        })
    }
}
