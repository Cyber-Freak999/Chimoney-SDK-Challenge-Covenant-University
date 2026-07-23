# Chimoney Rust SDK v0.2.0 — Code Quality Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Improve code quality of the Chimoney Rust SDK by adding proper error handling, typed request/response structs, connection pooling with retries, and methods on a client struct.

**Architecture:** Replace free functions with methods on a `ChimoneyClient` struct. Use `thiserror` for custom error types. Use `reqwest-middleware` + `reqwest-retry` for automatic retry handling. Define typed request/response structs using serde.

**Tech Stack:** Rust, reqwest, reqwest-middleware, reqwest-retry, thiserror, serde, serde_json

## Global Constraints

- Rust edition 2021 (current), consider upgrading to 2024
- Minimum supported Rust version: 1.70 (for thiserror 1.x)
- No `dotenv` dependency — API key passed to constructor
- All public functions return `Result<T, ChimoneyError>`
- Version bump to 0.2.0

---

## File Structure

```
src/
├── lib.rs           # MODIFY: Update exports
├── client.rs        # CREATE: ChimoneyClient + builder + methods
├── error.rs         # CREATE: ChimoneyError enum
├── middleware.rs     # CREATE: Retry middleware setup
├── types/
│   ├── mod.rs       # CREATE: Re-exports all types
│   ├── account.rs   # CREATE: Transaction, TransferRequest, TransferResponse
│   ├── payment.rs   # CREATE: PaymentRequest, PaymentResponse, PaymentVerification
│   ├── payout.rs    # CREATE: PayoutRequest, PayoutResponse
│   ├── redeem.rs    # CREATE: RedeemRequest, RedeemResponse
│   ├── subaccount.rs # CREATE: SubAccount, CreateSubAccountRequest
│   └── wallet.rs    # CREATE: Wallet, WalletTransferRequest
├── core/
│   └── api_client.rs # DELETE: Remove old client
├── account/
│   └── account.rs   # DELETE: Remove free functions
├── payment/
│   └── payment.rs   # DELETE: Remove free functions
├── payouts/
│   └── payouts.rs   # DELETE: Remove free functions
├── redeem/
│   └── redeem.rs    # DELETE: Remove free functions
├── subaccount/
│   └── subaccount.rs # DELETE: Remove free functions
├── wallet/
│   └── wallet.rs    # DELETE: Remove free functions
├── beneficiary/
│   └── beneficiary.rs # DELETE: Remove free functions
└── info/
    └── info.rs      # DELETE: Remove free functions
```

---

## Task 1: Update Cargo.toml

**Files:**
- Modify: `Cargo.toml`

**Interfaces:**
- Consumes: None
- Produces: Updated dependency list for v0.2.0

- [ ] **Step 1: Update Cargo.toml with new dependencies**

```toml
[package]
name = "chimoney_rust_sdk"
version = "0.2.0"
edition = "2021"
description = "Rust SDK for the Chimoney API"
license = "MIT"

[dependencies]
reqwest = { version = "0.11", features = ["json"] }
reqwest-middleware = "0.2"
reqwest-retry = "0.2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
log = "0.4"

[dev-dependencies]
tokio = { version = "1", features = ["full"] }
```

- [ ] **Step 2: Run cargo check to verify dependencies**

Run: `cargo check`
Expected: Dependencies compile successfully

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "chore: update dependencies for v0.2.0"
```

---

## Task 2: Create Error Types

**Files:**
- Create: `src/error.rs`

**Interfaces:**
- Consumes: None
- Produces: `ChimoneyError` enum, `Result<T>` type alias

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

---

## Task 3: Create Request/Response Types

**Files:**
- Create: `src/types/mod.rs`
- Create: `src/types/account.rs`
- Create: `src/types/payment.rs`
- Create: `src/types/payout.rs`
- Create: `src/types/redeem.rs`
- Create: `src/types/subaccount.rs`
- Create: `src/types/wallet.rs`

**Interfaces:**
- Consumes: None
- Produces: Typed request/response structs for all API endpoints

- [ ] **Step 1: Create src/types/mod.rs**

```rust
mod account;
mod payment;
mod payout;
mod redeem;
mod subaccount;
mod wallet;

pub use account::*;
pub use payment::*;
pub use payout::*;
pub use redeem::*;
pub use subaccount::*;
pub use wallet::*;
```

- [ ] **Step 2: Create src/types/account.rs**

```rust
use serde::{Deserialize, Serialize};

/// A Chimoney transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Request to transfer between accounts.
#[derive(Debug, Clone, Serialize)]
pub struct TransferRequest {
    /// The receiver's ID or email.
    pub receiver: String,
    /// Amount in USD.
    pub value_in_usd: f64,
    /// Optional sub-account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<String>,
}

/// Response from a transfer request.
#[derive(Debug, Clone, Deserialize)]
pub struct TransferResponse {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
}

/// Request to initiate a Chimoney transaction.
#[derive(Debug, Clone, Serialize)]
pub struct InitiateChimoneyRequest {
    /// Receiver's email or ID.
    pub receiver: String,
    /// Amount in USD.
    pub value_in_usd: f64,
    /// Optional sub-account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<String>,
    /// Optional turn off notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_off_notification: Option<bool>,
}

/// Response from initiating a Chimoney transaction.
#[derive(Debug, Clone, Deserialize)]
pub struct InitiateChimoneyResponse {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
}

/// Request to delete unpaid transactions.
#[derive(Debug, Clone, Serialize)]
pub struct DeleteUnpaidTransactionRequest {
    /// The chi reference to delete.
    pub chi_ref: String,
}

/// Response from deleting unpaid transactions.
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteUnpaidTransactionResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
}
```

- [ ] **Step 3: Create src/types/payment.rs**

```rust
use serde::{Deserialize, Serialize};

/// Request to initiate a payment.
#[derive(Debug, Clone, Serialize)]
pub struct PaymentRequest {
    /// Payer's email address.
    pub email: String,
    /// Amount in USD.
    pub amount: f64,
    /// Redirect URL after payment.
    pub redirect_url: String,
    /// Optional sub-account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<String>,
}

/// Response from initiating a payment.
#[derive(Debug, Clone, Deserialize)]
pub struct PaymentResponse {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub checkout_url: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

/// Payment verification result.
#[derive(Debug, Clone, Deserialize)]
pub struct PaymentVerification {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}
```

- [ ] **Step 4: Create src/types/payout.rs**

```rust
use serde::{Deserialize, Serialize};

/// Base payout request with common fields.
#[derive(Debug, Clone, Serialize)]
pub struct PayoutRequest {
    /// Sub-account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<String>,
    /// Turn off notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_off_notification: Option<bool>,
}

/// Bank payout request.
#[derive(Debug, Clone, Serialize)]
pub struct BankPayoutRequest {
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of bank transfers.
    pub transfers: Vec<BankTransfer>,
}

/// A single bank transfer.
#[derive(Debug, Clone, Serialize)]
pub struct BankTransfer {
    /// Bank code.
    pub bank_code: String,
    /// Account number.
    pub account_number: String,
    /// Amount in USD.
    pub amount: f64,
    /// Currency.
    pub currency: String,
    /// Country code.
    pub country_code: String,
    /// Beneficiary name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_name: Option<String>,
}

/// Airtime payout request.
#[derive(Debug, Clone, Serialize)]
pub struct AirtimePayoutRequest {
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of airtime transfers.
    pub transfers: Vec<AirtimeTransfer>,
}

/// A single airtime transfer.
#[derive(Debug, Clone, Serialize)]
pub struct AirtimeTransfer {
    /// Phone number.
    pub phone_number: String,
    /// Amount in USD.
    pub amount: f64,
    /// Country code.
    pub country_code: String,
}

/// Chimoney payout request.
#[derive(Debug, Clone, Serialize)]
pub struct ChimoneyPayoutRequest {
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of Chimoney transfers.
    pub transfers: Vec<ChimoneyTransfer>,
}

/// A single Chimoney transfer.
#[derive(Debug, Clone, Serialize)]
pub struct ChimoneyTransfer {
    /// Receiver email or ID.
    pub receiver: String,
    /// Amount in USD.
    pub value_in_usd: f64,
}

/// Mobile money payout request.
#[derive(Debug, Clone, Serialize)]
pub struct MobileMoneyPayoutRequest {
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of mobile money transfers.
    pub transfers: Vec<MobileMoneyTransfer>,
}

/// A single mobile money transfer.
#[derive(Debug, Clone, Serialize)]
pub struct MobileMoneyTransfer {
    /// Phone number.
    pub phone_number: String,
    /// Amount in USD.
    pub amount: f64,
    /// Country code.
    pub country_code: String,
    /// Mobile money provider code.
    pub provider_code: String,
}

/// Gift card payout request.
#[derive(Debug, Clone, Serialize)]
pub struct GiftCardPayoutRequest {
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of gift card transfers.
    pub transfers: Vec<GiftCardTransfer>,
}

/// A single gift card transfer.
#[derive(Debug, Clone, Serialize)]
pub struct GiftCardTransfer {
    /// Receiver email or ID.
    pub receiver: String,
    /// Amount in USD.
    pub value_in_usd: f64,
    /// Gift card provider.
    pub provider: String,
}

/// Interledger wallet payout request.
#[derive(Debug, Clone, Serialize)]
pub struct InterledgerPayoutRequest {
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of Interledger transfers.
    pub transfers: Vec<InterledgerTransfer>,
}

/// A single Interledger transfer.
#[derive(Debug, Clone, Serialize)]
pub struct InterledgerTransfer {
    /// Receiver address.
    pub receiver_address: String,
    /// Amount in USD.
    pub value_in_usd: f64,
}

/// Wallet payout request.
#[derive(Debug, Clone, Serialize)]
pub struct WalletPayoutRequest {
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of wallet transfers.
    pub transfers: Vec<WalletTransfer>,
}

/// A single wallet transfer.
#[derive(Debug, Clone, Serialize)]
pub struct WalletTransfer {
    /// Receiver email or ID.
    pub receiver: String,
    /// Amount in USD.
    pub value_in_usd: f64,
    /// Wallet ID.
    pub wallet_id: String,
}

/// Generic payout response.
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
}

/// Payout status response.
#[derive(Debug, Clone, Deserialize)]
pub struct PayoutStatusResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}
```

- [ ] **Step 5: Create src/types/redeem.rs**

```rust
use serde::{Deserialize, Serialize};

/// Base redeem request.
#[derive(Debug, Clone, Serialize)]
pub struct RedeemRequest {
    /// Sub-account ID.
    pub sub_account: String,
    /// Chi reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chi_ref: Option<String>,
    /// Turn off notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_off_notification: Option<bool>,
}

/// Airtime redeem request.
#[derive(Debug, Clone, Serialize)]
pub struct RedeemAirtimeRequest {
    #[serde(flatten)]
    pub base: RedeemRequest,
    /// Country code.
    pub country_to_send: String,
    /// Phone number.
    pub phone_number: String,
    /// Test mode flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
}

/// Chimoney redeem request.
#[derive(Debug, Clone, Serialize)]
pub struct RedeemChimoneyRequest {
    #[serde(flatten)]
    pub base: RedeemRequest,
    /// Chimoney key-value pairs.
    pub chimoneys: std::collections::HashMap<String, String>,
}

/// Gift card redeem request.
#[derive(Debug, Clone, Serialize)]
pub struct RedeemGiftCardRequest {
    #[serde(flatten)]
    pub base: RedeemRequest,
    /// Redeem options key-value pairs.
    pub redeem_options: std::collections::HashMap<String, String>,
}

/// Mobile money redeem request.
#[derive(Debug, Clone, Serialize)]
pub struct RedeemMobileMoneyRequest {
    #[serde(flatten)]
    pub base: RedeemRequest,
    /// Redeem options key-value pairs.
    pub redeem_options: std::collections::HashMap<String, String>,
}

/// Generic redeem response.
#[derive(Debug, Clone, Deserialize)]
pub struct RedeemResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
}
```

- [ ] **Step 6: Create src/types/subaccount.rs**

```rust
use serde::{Deserialize, Serialize};

/// Sub-account details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAccount {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub last_name: Option<String>,
    #[serde(default)]
    pub phone_number: Option<String>,
}

/// Request to create a sub-account.
#[derive(Debug, Clone, Serialize)]
pub struct CreateSubAccountRequest {
    /// Sub-account name.
    pub name: String,
    /// First name.
    pub first_name: String,
    /// Last name.
    pub last_name: String,
    /// Email address.
    pub email: String,
    /// Phone number.
    pub phone_number: String,
}

/// Request to update a sub-account.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateSubAccountRequest {
    /// Sub-account ID.
    pub id: String,
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Metadata key-value pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::HashMap<String, String>>,
}

/// Response from sub-account operations.
#[derive(Debug, Clone, Deserialize)]
pub struct SubAccountResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}

/// List of sub-accounts.
#[derive(Debug, Clone, Deserialize)]
pub struct SubAccountList {
    pub status: String,
    #[serde(default)]
    pub data: Option<Vec<SubAccount>>,
}
```

- [ ] **Step 7: Create src/types/wallet.rs**

```rust
use serde::{Deserialize, Serialize};

/// Wallet details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub balance: Option<f64>,
}

/// List of wallets response.
#[derive(Debug, Clone, Deserialize)]
pub struct WalletList {
    pub status: String,
    #[serde(default)]
    pub data: Option<Vec<Wallet>>,
}

/// Request to lookup a wallet.
#[derive(Debug, Clone, Serialize)]
pub struct WalletLookupRequest {
    /// Wallet ID.
    pub wallet_id: String,
    /// Sub-account ID.
    pub sub_account: String,
}

/// Request to transfer between wallets.
#[derive(Debug, Clone, Serialize)]
pub struct WalletTransferRequest {
    /// Source wallet ID.
    pub wallet: String,
    /// Amount in USD.
    pub value_in_usd: f64,
    /// Sub-account ID.
    pub sub_account: String,
    /// Receiver email or ID.
    pub receiver: String,
}

/// Response from wallet operations.
#[derive(Debug, Clone, Deserialize)]
pub struct WalletResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}
```

- [ ] **Step 8: Run cargo check to verify types compile**

Run: `cargo check`
Expected: Compiles successfully

- [ ] **Step 9: Commit**

```bash
git add src/types/
git commit -m "feat: add typed request/response structs"
```

---

## Task 4: Create Retry Middleware

**Files:**
- Create: `src/middleware.rs`

**Interfaces:**
- Consumes: None
- Produces: `build_client` function

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

---

## Task 5: Create Client Struct

**Files:**
- Create: `src/client.rs`

**Interfaces:**
- Consumes: `ChimoneyError`, `Result`, `build_client`, types
- Produces: `ChimoneyClient` struct with all methods

- [ ] **Step 1: Create src/client.rs with struct and constructor**

```rust
use reqwest_middleware::Client;

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
    client: Client,
    api_key: String,
    base_url: String,
}

impl ChimoneyClient {
    /// Create a new ChimoneyClient with default settings.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your Chimoney API key
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
    pub fn http_client(&self) -> &Client {
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
            .map_err(ChimoneyError::RequestFailed)?;

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
            .map_err(ChimoneyError::RequestFailed)?;

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
            .map_err(ChimoneyError::RequestFailed)?;

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
            let json: serde_json::Value = serde_json::from_str(&text)
                .unwrap_or_else(|_| {
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
                let retry_after = json["retry_after"]
                    .as_u64()
                    .unwrap_or(60);
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
```

- [ ] **Step 2: Run cargo check to verify client compiles**

Run: `cargo check`
Expected: Compiles successfully

- [ ] **Step 3: Commit**

```bash
git add src/client.rs
git commit -m "feat: add ChimoneyClient with builder pattern"
```

---

## Task 6: Add Account Methods

**Files:**
- Modify: `src/client.rs`

**Interfaces:**
- Consumes: `ChimoneyClient`, account types
- Produces: Account methods on `ChimoneyClient`

- [ ] **Step 1: Add account methods to client.rs**

```rust
impl ChimoneyClient {
    // ... existing code ...

    /// Get transactions by account ID.
    pub async fn get_transactions(&self, account_id: &str) -> Result<Vec<crate::types::Transaction>> {
        let path = "/v0.2/accounts/transactions";
        let query = format!("accountId={}", account_id);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json["data"].clone())
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Get single transaction details.
    pub async fn get_transaction(&self, transaction_id: &str) -> Result<crate::types::Transaction> {
        let path = "/v0.2/accounts/transaction";
        let query = format!("id={}", transaction_id);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json["data"].clone())
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Get transaction by issue ID.
    pub async fn get_issue_id_transaction(&self, issue_id: &str) -> Result<serde_json::Value> {
        let path = "/v0.2/accounts/issue-id-transactions";
        let query = format!("issueId={}", issue_id);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Get public profile.
    pub async fn get_public_profile(&self) -> Result<serde_json::Value> {
        let path = "/v0.2/accounts/public-profile";
        let response = self.get(path, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Transfer between accounts.
    pub async fn transfer(
        &self,
        request: &crate::types::TransferRequest,
    ) -> Result<crate::types::TransferResponse> {
        let path = "/v0.2/accounts/transfer";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Initiate Chimoney transaction.
    pub async fn initiate_chimoney(
        &self,
        request: &crate::types::InitiateChimoneyRequest,
    ) -> Result<crate::types::InitiateChimoneyResponse> {
        let path = "/v0.2/payouts/initiate-chimoney";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Delete unpaid transactions.
    pub async fn delete_unpaid_transactions(
        &self,
        chi_ref: &str,
    ) -> Result<crate::types::DeleteUnpaidTransactionResponse> {
        let path = "/v0.2/accounts/delete-unpaid-transaction";
        let query = format!("chiRef={}", chi_ref);
        let response = self.delete(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }
}
```

- [ ] **Step 2: Run cargo check to verify methods compile**

Run: `cargo check`
Expected: Compiles successfully

- [ ] **Step 3: Commit**

```bash
git add src/client.rs
git commit -m "feat: add account methods to ChimoneyClient"
```

---

## Task 7: Add Payment Methods

**Files:**
- Modify: `src/client.rs`

**Interfaces:**
- Consumes: `ChimoneyClient`, payment types
- Produces: Payment methods on `ChimoneyClient`

- [ ] **Step 1: Add payment methods to client.rs**

```rust
impl ChimoneyClient {
    // ... existing code ...

    /// Initiate a payment.
    pub async fn initiate_payment(
        &self,
        request: &crate::types::PaymentRequest,
    ) -> Result<crate::types::PaymentResponse> {
        let path = "/v0.2/payment/initiate";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Verify a payment.
    pub async fn verify_payment(
        &self,
        issue_id: &str,
    ) -> Result<crate::types::PaymentVerification> {
        let path = "/v0.2/payment/verify";
        let query = format!("issueId={}", issue_id);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json["data"].clone())
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Simulate a payment (sandbox only).
    pub async fn simulate_payment(
        &self,
        issue_id: &str,
    ) -> Result<serde_json::Value> {
        let path = "/v0.2/payment/simulate";
        let query = format!("issueId={}", issue_id);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }
}
```

- [ ] **Step 2: Run cargo check to verify methods compile**

Run: `cargo check`
Expected: Compiles successfully

- [ ] **Step 3: Commit**

```bash
git add src/client.rs
git commit -m "feat: add payment methods to ChimoneyClient"
```

---

## Task 8: Add Payout Methods

**Files:**
- Modify: `src/client.rs`

**Interfaces:**
- Consumes: `ChimoneyClient`, payout types
- Produces: Payout methods on `ChimoneyClient`

- [ ] **Step 1: Add payout methods to client.rs**

```rust
impl ChimoneyClient {
    // ... existing code ...

    /// Payout via bank transfer.
    pub async fn payout_bank(
        &self,
        request: &crate::types::BankPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        let path = "/v0.2/payouts/bank";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Payout via airtime.
    pub async fn payout_airtime(
        &self,
        request: &crate::types::AirtimePayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        let path = "/v0.2/payouts/airtime";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Payout via Chimoney.
    pub async fn payout_chimoney(
        &self,
        request: &crate::types::ChimoneyPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        let path = "/v0.2/payouts/chimoney";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Payout via mobile money.
    pub async fn payout_mobile_money(
        &self,
        request: &crate::types::MobileMoneyPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        let path = "/v0.2/payouts/mobile-money";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Payout via gift card.
    pub async fn payout_giftcard(
        &self,
        request: &crate::types::GiftCardPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        let path = "/v0.2/payouts/gift-card";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Payout via Interledger wallet.
    pub async fn payout_interledger(
        &self,
        request: &crate::types::InterledgerPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        let path = "/v0.2/payouts/interledger-wallet";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Payout via wallet.
    pub async fn payout_wallet(
        &self,
        request: &crate::types::WalletPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        let path = "/v0.2/payouts/wallet";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Check payout status.
    pub async fn check_payout_status(
        &self,
        chi_ref: &str,
    ) -> Result<crate::types::PayoutStatusResponse> {
        let path = "/v0.2/payouts/status";
        let query = format!("chiRef={}", chi_ref);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }
}
```

- [ ] **Step 2: Run cargo check to verify methods compile**

Run: `cargo check`
Expected: Compiles successfully

- [ ] **Step 3: Commit**

```bash
git add src/client.rs
git commit -m "feat: add payout methods to ChimoneyClient"
```

---

## Task 9: Add Redeem, SubAccount, Wallet, and Info Methods

**Files:**
- Modify: `src/client.rs`

**Interfaces:**
- Consumes: `ChimoneyClient`, all remaining types
- Produces: All remaining methods on `ChimoneyClient`

- [ ] **Step 1: Add redeem methods**

```rust
impl ChimoneyClient {
    // ... existing code ...

    /// Redeem airtime.
    pub async fn redeem_airtime(
        &self,
        request: &crate::types::RedeemAirtimeRequest,
    ) -> Result<crate::types::RedeemResponse> {
        let path = "/v0.2/redeem/airtime";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Redeem Chimoney.
    pub async fn redeem_chimoney(
        &self,
        request: &crate::types::RedeemChimoneyRequest,
    ) -> Result<crate::types::RedeemResponse> {
        let path = "/v0.2/redeem/chimoney";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Redeem gift card.
    pub async fn redeem_giftcard(
        &self,
        request: &crate::types::RedeemGiftCardRequest,
    ) -> Result<crate::types::RedeemResponse> {
        let path = "/v0.2/redeem/gift-card";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Redeem mobile money.
    pub async fn redeem_mobile_money(
        &self,
        request: &crate::types::RedeemMobileMoneyRequest,
    ) -> Result<crate::types::RedeemResponse> {
        let path = "/v0.2/redeem/mobile-money";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }
}
```

- [ ] **Step 2: Add subaccount methods**

```rust
impl ChimoneyClient {
    // ... existing code ...

    /// Create a sub-account.
    pub async fn create_sub_account(
        &self,
        request: &crate::types::CreateSubAccountRequest,
    ) -> Result<crate::types::SubAccountResponse> {
        let path = "/v0.2/sub-account/create";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Update a sub-account.
    pub async fn update_sub_account(
        &self,
        request: &crate::types::UpdateSubAccountRequest,
    ) -> Result<crate::types::SubAccountResponse> {
        let path = "/v0.2/sub-account/update";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Delete a sub-account.
    pub async fn delete_sub_account(
        &self,
        sub_account_id: &str,
    ) -> Result<crate::types::SubAccountResponse> {
        let path = "/v0.2/sub-account/delete";
        let query = format!("id={}", sub_account_id);
        let response = self.delete(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Get sub-account details.
    pub async fn get_sub_account(
        &self,
        sub_account_id: &str,
    ) -> Result<serde_json::Value> {
        let path = "/v0.2/sub-account/get";
        let query = format!("id={}", sub_account_id);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// List all sub-accounts.
    pub async fn list_sub_accounts(
        &self,
    ) -> Result<Vec<serde_json::Value>> {
        let path = "/v0.2/sub-account/list";
        let response = self.get(path, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json["data"].clone())
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }
}
```

- [ ] **Step 3: Add wallet methods**

```rust
impl ChimoneyClient {
    // ... existing code ...

    /// List wallets.
    pub async fn list_wallets(
        &self,
        sub_account: &str,
    ) -> Result<crate::types::WalletList> {
        let path = "/v0.2/wallets/list";
        let body = serde_json::json!({ "subAccount": sub_account }).to_string();
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Lookup a wallet.
    pub async fn lookup_wallet(
        &self,
        request: &crate::types::WalletLookupRequest,
    ) -> Result<crate::types::WalletResponse> {
        let path = "/v0.2/wallets/lookup";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Transfer between wallets.
    pub async fn transfer_between_wallets(
        &self,
        request: &crate::types::WalletTransferRequest,
    ) -> Result<crate::types::WalletResponse> {
        let path = "/v0.2/wallets/transfer";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }
}
```

- [ ] **Step 4: Add info methods**

```rust
impl ChimoneyClient {
    // ... existing code ...

    /// Get airtime countries.
    pub async fn get_airtime_countries(&self) -> Result<serde_json::Value> {
        let path = "/v0.2/info/airtime-countries";
        let response = self.get(path, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Get assets by country code.
    pub async fn get_assets(&self, country_code: &str) -> Result<serde_json::Value> {
        let path = "/v0.2/info/assets";
        let query = format!("countryCode={}", country_code);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Get banks by country code.
    pub async fn get_banks(&self, country_code: &str) -> Result<serde_json::Value> {
        let path = "/v0.2/info/country-banks";
        let query = format!("countryCode={}", country_code);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Get bank branches.
    pub async fn get_bank_branches(&self, bank_code: &str) -> Result<serde_json::Value> {
        let path = "/v0.2/info/bank-branches";
        let query = format!("bankCode={}", bank_code);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Get exchange rates.
    pub async fn get_exchange_rates(&self) -> Result<serde_json::Value> {
        let path = "/v0.2/info/exchange-rates";
        let response = self.get(path, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Convert local currency to USD.
    pub async fn local_to_usd(
        &self,
        currency: &str,
        amount: &str,
    ) -> Result<serde_json::Value> {
        let path = "/v0.2/info/local-amount-to-usd";
        let query = format!("originCurrency={}&amountInOriginCurrency={}", currency, amount);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Convert USD to local currency.
    pub async fn usd_to_local(
        &self,
        currency: &str,
        amount: &str,
    ) -> Result<serde_json::Value> {
        let path = "/v0.2/info/usd-amount-in-local";
        let query = format!("destinationCurrency={}&amountInUSD={}", currency, amount);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Get mobile money codes.
    pub async fn get_mobile_money_codes(&self) -> Result<serde_json::Value> {
        let path = "/v0.2/info/mobile-money-codes";
        let response = self.get(path, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }

    /// Verify bank account.
    pub async fn verify_bank_account(
        &self,
        country_code: &str,
        bank_code: &str,
        account_number: &str,
    ) -> Result<serde_json::Value> {
        let path = "/v0.2/info/verify-bank-account";
        let body = serde_json::json!({
            "verifyAccountNumbers": [{
                "countryCode": country_code,
                "account_bank": bank_code,
                "account_number": account_number
            }]
        }).to_string();
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }
}
```

- [ ] **Step 5: Run cargo check to verify all methods compile**

Run: `cargo check`
Expected: Compiles successfully

- [ ] **Step 6: Commit**

```bash
git add src/client.rs
git commit -m "feat: add redeem, subaccount, wallet, and info methods"
```

---

## Task 10: Update lib.rs Exports

**Files:**
- Modify: `src/lib.rs`

**Interfaces:**
- Consumes: All modules
- Produces: Public API exports

- [ ] **Step 1: Update src/lib.rs**

```rust
//! Chimoney Rust SDK
//!
//! A Rust client library for the Chimoney API.
//!
//! # Example
//!
//! ```rust
//! use chimoney_rust_sdk::{ChimoneyClient, error::Result};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let client = ChimoneyClient::new("your_api_key")?;
//! let transactions = client.get_transactions("account_id").await?;
//! # Ok(())
//! # }
//! ```

pub mod client;
pub mod error;
pub mod middleware;
pub mod types;

pub use client::{ChimoneyClient, ChimoneyClientBuilder};
pub use error::{ChimoneyError, Result};
```

- [ ] **Step 2: Run cargo check to verify exports compile**

Run: `cargo check`
Expected: Compiles successfully

- [ ] **Step 3: Commit**

```bash
git add src/lib.rs
git commit -m "feat: update lib.rs exports for v0.2.0"
```

---

## Task 11: Remove Old Code

**Files:**
- Delete: `src/core/api_client.rs`
- Delete: `src/account/account.rs`
- Delete: `src/payment/payment.rs`
- Delete: `src/payouts/payouts.rs`
- Delete: `src/redeem/redeem.rs`
- Delete: `src/subaccount/subaccount.rs`
- Delete: `src/wallet/wallet.rs`
- Delete: `src/beneficiary/beneficiary.rs`
- Delete: `src/info/info.rs`
- Delete: `src/core/mod.rs` (if exists)
- Delete: `src/account/mod.rs` (if exists)
- Delete: `src/payment/mod.rs` (if exists)
- Delete: `src/payouts/mod.rs` (if exists)
- Delete: `src/redeem/mod.rs` (if exists)
- Delete: `src/subaccount/mod.rs` (if exists)
- Delete: `src/wallet/mod.rs` (if exists)
- Delete: `src/beneficiary/mod.rs` (if exists)
- Delete: `src/info/mod.rs` (if exists)

**Interfaces:**
- Consumes: None
- Produces: Cleaned up codebase

- [ ] **Step 1: Remove old module files**

Run:
```bash
rm -rf src/core src/account src/payment src/payouts src/redeem src/subaccount src/wallet src/beneficiary src/info
```

- [ ] **Step 2: Run cargo check to verify codebase compiles**

Run: `cargo check`
Expected: Compiles successfully (no references to old modules)

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "chore: remove old code and module structure"
```

---

## Task 12: Add Documentation and Examples

**Files:**
- Create: `examples/basic_usage.rs`
- Create: `CHANGELOG.md`
- Modify: `Cargo.toml` (add examples section)

**Interfaces:**
- Consumes: All public API
- Produces: Working examples and changelog

- [ ] **Step 1: Create examples/basic_usage.rs**

```rust
use chimoney_rust_sdk::{ChimoneyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize client with API key
    let client = ChimoneyClient::new("your_api_key")?;

    // Get airtime countries
    let countries = client.get_airtime_countries().await?;
    println!("Airtime countries: {}", countries);

    // Get exchange rates
    let rates = client.get_exchange_rates().await?;
    println!("Exchange rates: {}", rates);

    // Get banks in Nigeria
    let banks = client.get_banks("NG").await?;
    println!("Nigerian banks: {}", banks);

    Ok(())
}
```

- [ ] **Step 2: Create CHANGELOG.md**

```markdown
# Changelog

## [0.2.0] - 2026-07-23

### Added
- `ChimoneyClient` struct with builder pattern
- Custom `ChimoneyError` enum with `thiserror`
- Typed request/response structs for all endpoints
- Retry middleware with exponential backoff
- Connection pooling via `reqwest-middleware`
- Sandbox support via `ChimoneyClient::new_sandbox()`
- Documentation and examples

### Changed
- All functions now return `Result<T, ChimoneyError>` instead of `Result<String, Box<dyn Error>>`
- API methods are now on `ChimoneyClient` struct instead of free functions
- Responses are now deserialized to typed structs

### Removed
- `APIClient` struct
- Free functions for all endpoints
- `dotenv` dependency

## [0.1.0] - Initial release
```

- [ ] **Step 3: Update Cargo.toml with examples**

```toml
[[example]]
name = "basic_usage"
path = "examples/basic_usage.rs"
```

- [ ] **Step 4: Run cargo check and verify example compiles**

Run: `cargo check --examples`
Expected: Compiles successfully

- [ ] **Step 5: Commit**

```bash
git add examples/ CHANGELOG.md Cargo.toml
git commit -m "docs: add examples and changelog"
```

---

## Task 13: Final Verification

**Files:**
- None (verification only)

**Interfaces:**
- Consumes: All tasks above
- Produces: Verified, working SDK

- [ ] **Step 1: Run cargo check**

Run: `cargo check`
Expected: Compiles successfully

- [ ] **Step 2: Run cargo test**

Run: `cargo test`
Expected: All tests pass (or no tests if none exist yet)

- [ ] **Step 3: Run cargo build**

Run: `cargo build`
Expected: Builds successfully

- [ ] **Step 4: Run cargo doc**

Run: `cargo doc --no-deps`
Expected: Documentation generates successfully

- [ ] **Step 5: Verify example compiles and runs**

Run: `cargo run --example basic_usage`
Expected: Runs (may fail with API key error, but compiles)

- [ ] **Step 6: Final commit**

```bash
git add -A
git commit -m "chore: verify v0.2.0 builds and tests pass"
```

---

## Summary

**Total tasks:** 13
**Estimated time:** 45-60 minutes

**Key deliverables:**
1. Custom error types with `thiserror`
2. `ChimoneyClient` struct with builder pattern
3. Typed request/response structs
4. Retry middleware with exponential backoff
5. All API methods on client struct
6. Documentation and examples
7. Cleaned up codebase (old code removed)

**Breaking changes:**
- `APIClient` removed
- Free functions removed
- `dotenv` dependency removed
- Responses are now typed structs
