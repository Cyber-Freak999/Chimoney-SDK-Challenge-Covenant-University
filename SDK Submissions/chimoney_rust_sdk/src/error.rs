use thiserror::Error;

/// Errors that can occur when using the Chimoney SDK.
#[derive(Debug, Error)]
pub enum ChimoneyError {
    /// API key was not provided.
    #[error("API key not provided")]
    ApiKeyMissing,

    /// API key is empty.
    #[error("API key cannot be empty")]
    ApiKeyEmpty,

    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    /// Request middleware error.
    #[error("Request middleware error: {0}")]
    MiddlewareError(#[from] reqwest_middleware::Error),

    /// API returned an error response.
    #[error("API error {status}: {message}")]
    ApiError {
        status: u16,
        message: String,
    },

    /// Rate limited by the API.
    #[error("Rate limited, retry after {retry_after} seconds")]
    RateLimited {
        retry_after: u64,
    },

    /// Failed to parse response.
    #[error("Failed to parse response: {0}")]
    ParseError(String),

    /// Response missing required field.
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Invalid URL format.
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}

/// Result type alias for Chimoney SDK operations.
pub type Result<T> = std::result::Result<T, ChimoneyError>;
