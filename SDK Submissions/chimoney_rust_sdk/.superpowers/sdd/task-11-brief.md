# Task 11: Remove Old Code

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

## Steps

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

## Notes

- This task removes all the old code that has been replaced
- The old modules (account, payment, payouts, redeem, subaccount, wallet, beneficiary, info) are no longer needed
- The core/api_client.rs is replaced by the new client.rs
- After this task, the codebase will only contain the new v0.2.0 code
- This is a breaking change (old API removed)
