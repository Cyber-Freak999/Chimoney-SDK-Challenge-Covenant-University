# Task 8: Add Payout Methods — Report

## Summary

Added 8 payout-related methods to `ChimoneyClient` in `src/client.rs`.

## Methods Added

| Method | Endpoint | Request Type |
|--------|----------|-------------|
| `payout_bank` | `POST /v0.2/payouts/bank` | `BankPayoutRequest` |
| `payout_airtime` | `POST /v0.2/payouts/airtime` | `AirtimePayoutRequest` |
| `payout_chimoney` | `POST /v0.2/payouts/chimoney` | `ChimoneyPayoutRequest` |
| `payout_mobile_money` | `POST /v0.2/payouts/mobile-money` | `MobileMoneyPayoutRequest` |
| `payout_giftcard` | `POST /v0.2/payouts/gift-card` | `GiftCardPayoutRequest` |
| `payout_interledger` | `POST /v0.2/payouts/interledger-wallet` | `InterledgerPayoutRequest` |
| `payout_wallet` | `POST /v0.2/payouts/wallet` | `WalletPayoutRequest` |
| `check_payout_status` | `GET /v0.2/payouts/status` | `chi_ref: &str` query param |

## Verification

- `cargo check` passed — no errors, no warnings.

## Commit

- `7d79475` feat: add payout methods to ChimoneyClient
