use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
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
/// A `ClientWithMiddleware` with retry middleware configured.
pub fn build_client(max_retries: u32, timeout_secs: u64) -> Result<ClientWithMiddleware> {
    let retry_policy = ExponentialBackoff::builder()
        .build_with_max_retries(max_retries);

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
