# Task 6: Add Account Methods

**Status:** DONE

## Summary

Added 7 account-related methods to `ChimoneyClient` in `src/client.rs`.

## Methods Added

| Method | HTTP | Endpoint | Return Type |
|--------|------|----------|-------------|
| `get_transactions` | GET | `/v0.2/accounts/transactions` | `Vec<Transaction>` |
| `get_transaction` | GET | `/v0.2/accounts/transaction` | `Transaction` |
| `get_issue_id_transaction` | GET | `/v0.2/accounts/issue-id-transactions` | `serde_json::Value` |
| `get_public_profile` | GET | `/v0.2/accounts/public-profile` | `serde_json::Value` |
| `transfer` | POST | `/v0.2/accounts/transfer` | `TransferResponse` |
| `initiate_chimoney` | POST | `/v0.2/payouts/initiate-chimoney` | `InitiateChimoneyResponse` |
| `delete_unpaid_transactions` | DELETE | `/v0.2/accounts/delete-unpaid-transaction` | `DeleteUnpaidTransactionResponse` |

## Verification

- `cargo check`: passes cleanly (0 warnings, 0 errors)
- Commit: `95151b0` — `feat: add account methods to ChimoneyClient`
