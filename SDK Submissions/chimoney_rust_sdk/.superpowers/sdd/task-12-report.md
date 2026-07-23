# Task 12: Add Documentation and Examples — Report

## Status: DONE

## Changes Made

1. **Created `examples/basic_usage.rs`** — Demonstrates initializing `ChimoneyClient`, fetching airtime countries, exchange rates, and Nigerian banks.
2. **Created `CHANGELOG.md`** — Documents v0.2.0 breaking changes (typed structs, builder pattern, error handling) and v0.1.0 initial release.
3. **Updated `Cargo.toml`** — Added `[[example]]` section for `basic_usage`.

## Verification

`cargo check --examples` passed successfully — example compiles without errors.

## Commits

- `4ad2629` — docs: add examples and changelog
