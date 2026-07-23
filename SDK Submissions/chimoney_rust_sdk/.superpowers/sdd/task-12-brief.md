# Task 12: Add Documentation and Examples

**Files:**
- Create: `examples/basic_usage.rs`
- Create: `CHANGELOG.md`
- Modify: `Cargo.toml` (add examples section)

**Interfaces:**
- Consumes: All public API
- Produces: Working examples and changelog

## Steps

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

## Notes

- This task adds documentation and examples for the SDK
- The example demonstrates basic usage of the client
- The changelog documents the breaking changes in v0.2.0
- This completes the documentation for the release
