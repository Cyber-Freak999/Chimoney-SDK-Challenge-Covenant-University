# Task 2: Create Error Types

**Files:**
- Create: `src/error.rs`

**Interfaces:**
- Consumes: None
- Produces: `ChimoneyError` enum, `Result<T>` type alias

## Steps

- [ ] **Step 1: Create src/error.rs with ChimoneyError enum**

```rust
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
```

- [ ] **Step 2: Run cargo check to verify error types compile**

Run: `cargo check`
Expected: Compiles successfully

- [ ] **Step 3: Commit**

```bash
git add src/error.rs
git commit -m "feat: add ChimoneyError enum with thiserror"
```

## Notes

- This task creates the error types that will be used by all other modules
- The `ChimoneyError` enum covers all failure modes (auth, network, API errors, parsing)
- Users can match on specific error variants for different handling
- `#[from]` on `RequestFailed` and `MiddlewareError` enables ergonomic `?` usage
- Type alias `Result<T>` keeps function signatures clean
- This is a foundation task - other tasks depend on it
