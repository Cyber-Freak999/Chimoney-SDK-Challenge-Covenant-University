# Task 1: Update Cargo.toml

**Files:**
- Modify: `Cargo.toml`

**Interfaces:**
- Consumes: None
- Produces: Updated dependency list for v0.2.0

## Steps

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

- [ ] **Step 2: Run cargo check to verify dependencies compile**

Run: `cargo check`
Expected: Dependencies compile successfully

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "chore: update dependencies for v0.2.0"
```

## Notes

- This is the first task in the code quality improvement plan
- No other tasks depend on this, but it must be done first
- The old `dotenv` dependency is removed (API key will be passed to constructor)
- New dependencies: `thiserror`, `reqwest-middleware`, `reqwest-retry`
- Version bumped from 0.1.0 to 0.2.0
