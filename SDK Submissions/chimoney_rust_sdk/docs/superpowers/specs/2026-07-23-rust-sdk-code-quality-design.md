# Chimoney Rust SDK v0.2.0 — Code Quality Improvements

## Overview

This document describes the design for improving code quality in the Chimoney Rust SDK. The SDK is a hackathon project being improved for learning purposes, with potential future importance.

**Current version:** 0.1.0  
**Target version:** 0.2.0  
**Scope:** Code quality improvements only (no new API endpoints)

## Problem Statement

The current SDK has several code quality issues:

1. **Error Handling:** All functions use `Box<dyn std::error::Error>` — users cannot match on specific error types
2. **API Design:** Free functions require passing the client on every call — not idiomatic for Rust SDKs
3. **Performance:** No connection pooling or retry logic for transient failures
4. **Code Style:** Uses `&bool` instead of `bool`, inconsistent documentation

## Design Decisions

### 1. Error Handling — `thiserror`

**Decision:** Use `thiserror` crate to define a custom error enum.

**Rationale:**
- Industry standard for Rust libraries
- Compile-time error type generation
- Users can match on specific error variants
- No runtime overhead

**Implementation:**

```rust
// src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChimoneyError {
    #[error("API key not found")]
    ApiKeyMissing,
    
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    
    #[error("API returned error: {status} - {message}")]
    ApiError { status: u16, message: String },
    
    #[error("Rate limited, retry after {retry_after}s")]
    RateLimited { retry_after: u64 },
    
    #[error("Invalid response format: {0}")]
    ParseError(String),
}

pub type Result<T> = std::result::Result<T, ChimoneyError>;
```

### 2. API Design — Methods on Client Struct

**Decision:** Replace free functions with methods on a `ChimoneyClient` struct.

**Rationale:**
- More discoverable (IDE autocomplete)
- Client holds configuration and HTTP client
- Standard pattern for Rust SDKs (Stripe, AWS)

**Implementation:**

```rust
// src/client.rs
pub struct ChimoneyClient {
    client: reqwest_middleware::Client,
    api_key: String,
    base_url: String,
}

impl ChimoneyClient {
    pub fn new(api_key: impl Into<String>) -> Result<Self> { /* ... */ }
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self { /* ... */ }
    
    // Account methods
    pub async fn get_transactions(&self, account_id: &str) -> Result<Vec<Transaction>> { /* ... */ }
    pub async fn transfer(&self, req: TransferRequest) -> Result<TransferResponse> { /* ... */ }
    
    // Payment methods
    pub async fn initiate_payment(&self, req: PaymentRequest) -> Result<PaymentResponse> { /* ... */ }
    pub async fn verify_payment(&self, issue_id: &str) -> Result<PaymentVerification> { /* ... */ }
}
```

**Before vs After:**

```rust
// BEFORE: Free functions
let client = APIClient::new("key")?;
let transactions = get_transactions_by_account(&client, "account_id").await?;

// AFTER: Methods on client
let client = ChimoneyClient::new("key")?;
let transactions = client.get_transactions("account_id").await?;
```

### 3. Connection Pooling & Retries — reqwest-middleware

**Decision:** Use `reqwest-middleware` + `reqwest-retry` for automatic retry handling.

**Rationale:**
- Battle-tested middleware approach
- Automatic handling of transient failures (429, 500, 502, 503)
- Exponential backoff with jitter
- Configurable retry policies

**Implementation:**

```rust
// src/middleware.rs
use reqwest_middleware::{ClientBuilder, Client};
use reqwest_retry::{
    RetryTransientMiddleware,
    policies::ExponentialBackoff,
};

pub fn build_client_with_retry(max_retries: u32) -> Result<Client> {
    let retry_policy = ExponentialBackoff::builder()
        .max_n_retries(max_retries)
        .build();
    
    let client = ClientBuilder::new(reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    
    Ok(client)
}
```

**Retry behavior:**

| Status Code | Action | Backoff |
|-------------|--------|---------|
| 429 | Retry, respect `Retry-After` header | Header-specified |
| 500, 502, 503 | Retry with exponential backoff | 1s → 2s → 4s → 8s → 16s |
| 400, 401, 403 | Return error immediately | N/A |

### 4. Builder Pattern for Client Configuration

**Decision:** Add a builder for complex client configuration.

```rust
impl ChimoneyClient {
    pub fn builder(api_key: impl Into<String>) -> ChimoneyClientBuilder {
        ChimoneyClientBuilder::default()
            .api_key(api_key)
    }
}

pub struct ChimoneyClientBuilder {
    api_key: String,
    base_url: String,
    max_retries: u32,
    timeout_secs: u64,
}

impl ChimoneyClientBuilder {
    pub fn max_retries(mut self, n: u32) -> Self { self.max_retries = n; self }
    pub fn timeout(mut self, secs: u64) -> Self { self.timeout_secs = secs; self }
    pub fn base_url(mut self, url: impl Into<String>) -> Self { self.base_url = url.into(); self }
    
    pub fn build(self) -> Result<ChimoneyClient> { /* ... */ }
}
```

**Usage:**

```rust
// Simple
let client = ChimoneyClient::new("api_key")?;

// Custom configuration
let client = ChimoneyClient::builder("api_key")
    .max_retries(5)
    .timeout(60)
    .base_url("https://sandbox-api.chimoney.io")
    .build()?;
```

### 5. Typed Request/Response Structs

**Decision:** Define typed structs for all API requests and responses using serde.

**Rationale:**
- Type safety at compile time
- IDE autocomplete for field names
- Automatic serialization/deserialization
- Better documentation

**Implementation:**

```rust
// src/types/account.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct TransferRequest {
    pub receiver: String,
    pub value_in_usd: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TransferResponse {
    pub id: String,
    pub status: String,
}
```

## File Structure

```
src/
├── lib.rs           # Re-exports ChimoneyClient, error types, request/response types
├── client.rs        # ChimoneyClient struct + builder + all methods
├── error.rs         # ChimoneyError enum
├── middleware.rs     # Retry middleware setup
├── types/
│   ├── mod.rs       # Re-exports all types
│   ├── account.rs   # Transaction, TransferRequest, TransferResponse
│   ├── payment.rs   # PaymentRequest, PaymentResponse, PaymentVerification
│   ├── payout.rs    # PayoutRequest, PayoutResponse
│   ├── redeem.rs    # RedeemRequest, RedeemResponse
│   ├── subaccount.rs # SubAccount, CreateSubAccountRequest
│   └── wallet.rs    # Wallet, WalletTransferRequest
```

## Dependencies

**Added:**
```toml
thiserror = "1"
reqwest-middleware = "0.2"
reqwest-retry = "0.2"
```

**Removed:**
```toml
dotenv = "*"  # API key passed to constructor, not read from env
```

**Kept:**
```toml
reqwest = "0.11"  # Will be upgraded to 0.12 if needed
serde = { version = "1", features = ["derive"] }
serde_json = "1"
log = "0.4"
```

## Migration Plan

### Phase 1: Add new types and client (non-breaking)
- Create `error.rs` with `ChimoneyError` enum
- Create `types/` module with request/response structs
- Create `client.rs` with `ChimoneyClient` struct
- Create `middleware.rs` with retry setup

### Phase 2: Implement methods on client
- Implement all account methods
- Implement all payment methods
- Implement all payout methods
- Implement all redeem methods
- Implement all subaccount methods
- Implement all wallet methods
- Implement all info methods

### Phase 3: Remove old API
- Remove `core/api_client.rs`
- Remove all free functions
- Remove `dotenv` dependency
- Update `lib.rs` exports

### Phase 4: Update Cargo.toml
- Bump version to 0.2.0
- Update dependencies
- Add `rust-version = "1.70"` (or appropriate MSRV)

## Breaking Changes

| Change | Impact |
|--------|--------|
| `APIClient` removed | Users must use `ChimoneyClient` |
| Free functions removed | Users must call methods on client |
| `dotenv` removed | Users must pass API key explicitly |
| Responses are typed structs | Users must update from `String` |
| `&bool` changed to `bool` | Minor, easy to fix |

## Testing Strategy

- Unit tests for each module
- Integration tests against mock server (or sandbox API)
- Example code in `examples/` directory
- Documentation tests in doc comments

## Success Criteria

1. All functions return `Result<T, ChimoneyError>` instead of `Result<String, Box<dyn Error>>`
2. Client struct holds configuration and HTTP client
3. Retry middleware handles transient failures automatically
4. Typed request/response structs for all endpoints
5. Documentation tests compile and pass
6. Example code demonstrates proper usage
