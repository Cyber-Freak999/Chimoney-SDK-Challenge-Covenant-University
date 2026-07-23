# Task 4: Create Retry Middleware

**Files:**
- Create: `src/middleware.rs`

**Interfaces:**
- Consumes: None
- Produces: `build_client` function

## Steps

- [ ] **Step 1: Create src/middleware.rs**

```rust
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, Client};
use reqwest_retry::{
    policies::ExponentialBackoff,
    RetryTransientMiddleware,
};

use crate::error::{ChimoneyError, Result};

/// Build a reqwest client with retry middleware.
///
/// # Arguments
///
/// * `max_retries` - Maximum number of retry attempts
/// * `timeout_secs` - Request timeout in seconds
///
/// # Returns
///
/// A `Client` with retry middleware configured.
pub fn build_client(max_retries: u32, timeout_secs: u64) -> Result<Client> {
    let retry_policy = ExponentialBackoff::builder()
        .max_n_retries(max_retries)
        .build();

    let reqwest_client = Client::builder()
        .timeout(std::time::Duration::from_secs(timeout_secs))
        .build()
        .map_err(ChimoneyError::RequestFailed)?;

    let client = ClientBuilder::new(reqwest_client)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    Ok(client)
}

/// Default retry count.
pub const DEFAULT_MAX_RETRIES: u32 = 3;

/// Default timeout in seconds.
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;
```

- [ ] **Step 2: Run cargo check to verify middleware compiles**

Run: `cargo check`
Expected: Compiles successfully

- [ ] **Step 3: Commit**

```bash
git add src/middleware.rs
git commit -m "feat: add retry middleware with exponential backoff"
```

## Notes

- This task creates the retry middleware setup
- Uses `reqwest-middleware` with `reqwest-retry` for automatic retry handling
- Exponential backoff policy: retries transient failures (429, 500, 502, 503)
- Default: 3 retries, 30 second timeout
- The `build_client` function returns a `Result<Client>` for error handling
- This is a foundation task - the client struct will use this
