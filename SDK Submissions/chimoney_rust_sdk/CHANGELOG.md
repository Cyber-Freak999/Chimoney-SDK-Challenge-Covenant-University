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
