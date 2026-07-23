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
