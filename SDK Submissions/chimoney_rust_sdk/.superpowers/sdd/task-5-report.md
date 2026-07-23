# Task 5: Create Client Struct — Report

## Status: DONE

## What was implemented

Created `src/client.rs` with:

- **`ChimoneyClient` struct** — holds `ClientWithMiddleware`, `api_key`, and `base_url`
- **Constructor methods**: `new()`, `new_sandbox()`, `builder()`
- **Accessor methods**: `http_client()`, `base_url()`
- **Internal HTTP methods**: `get()`, `post()`, `delete()` — private, for use by future API method tasks
- **`handle_response()`** — parses API responses, handles errors and 429 rate limits
- **`ChimoneyClientBuilder`** — builder pattern with `base_url()`, `max_retries()`, `timeout()`, `build()` methods

Also updated `src/lib.rs` to declare `pub mod client` and re-export `ChimoneyClient`, `ChimoneyClientBuilder`, `ChimoneyError`, and `Result`.

## Deviations from task brief

The task brief specified `use reqwest_middleware::Client` but the correct type is `ClientWithMiddleware` (as returned by `build_client()`). Fixed by using `ClientWithMiddleware` for the struct field and `http_client()` return type.

Similarly, `.send()` on `ClientWithMiddleware` returns `Result<_, reqwest_middleware::Error>`, not `reqwest::Error`. Updated the three HTTP methods to use `ChimoneyError::MiddlewareError` instead of `ChimoneyError::RequestFailed` for the `.send().await` calls.

## Verification

`cargo check` passes with only expected dead_code warnings (private methods unused until future API tasks add call sites).

## Commit

- `19eeb98` feat: add ChimoneyClient with builder pattern
