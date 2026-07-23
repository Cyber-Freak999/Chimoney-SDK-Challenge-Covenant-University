# Task 10: Update lib.rs Exports

**Status:** DONE

## What was done

Updated `src/lib.rs` to:
- Add module-level documentation with usage example
- Export the new modules: `client`, `error`, `middleware`, `types`
- Re-export `ChimoneyClient`, `ChimoneyClientBuilder`, `ChimoneyError`, and `Result`

## Verification

`cargo check` passed successfully (compiled in 2.04s).

## Commit

`2b63486` feat: update lib.rs exports for v0.2.0

## Notes

The old modules (account, beneficiary, core, info, payment, payouts, redeem, subaccount, wallet) were removed as part of the cleanup, leaving only the new v0.2.0 module structure.
