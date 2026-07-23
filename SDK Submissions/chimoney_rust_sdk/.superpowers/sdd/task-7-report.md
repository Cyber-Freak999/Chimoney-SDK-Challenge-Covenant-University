# Task 7 Report: Add Payment Methods

**Status:** DONE

## What Was Done

Added three payment methods to `ChimoneyClient` in `src/client.rs`:

- `initiate_payment(&self, request: &PaymentRequest) -> Result<PaymentResponse>` — POST to `/v0.2/payment/initiate`
- `verify_payment(&self, issue_id: &str) -> Result<PaymentVerification>` — GET to `/v0.2/payment/verify`
- `simulate_payment(&self, issue_id: &str) -> Result<serde_json::Value>` — GET to `/v0.2/payment/simulate`

All methods follow the same pattern as existing account methods (JSON serialization, error handling via `ChimoneyError::ParseError`, response extraction from `data` field where applicable).

## Verification

`cargo check` passed with no warnings or errors.

## Commit

- `59b7f2a` — feat: add payment methods to ChimoneyClient
