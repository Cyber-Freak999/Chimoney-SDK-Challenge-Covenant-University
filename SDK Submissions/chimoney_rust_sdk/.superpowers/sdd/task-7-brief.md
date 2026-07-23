# Task 7: Add Payment Methods

**Files:**
- Modify: `src/client.rs`

**Interfaces:**
- Consumes: `ChimoneyClient`, payment types
- Produces: Payment methods on `ChimoneyClient`

## Steps

- [ ] **Step 1: Add payment methods to client.rs**

```rust
impl ChimoneyClient {
    // ... existing code ...

    /// Initiate a payment.
    pub async fn initiate_payment(
        &self,
        request: &crate::types::PaymentRequest,
    ) -> Result<crate::types::PaymentResponse> {
        let path = "/v0.2/payment/initiate";
        let body = serde_json::to_string(request)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.post(path, &body, None).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Verify a payment.
    pub async fn verify_payment(
        &self,
        issue_id: &str,
    ) -> Result<crate::types::PaymentVerification> {
        let path = "/v0.2/payment/verify";
        let query = format!("issueId={}", issue_id);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        serde_json::from_value(json["data"].clone())
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    /// Simulate a payment (sandbox only).
    pub async fn simulate_payment(
        &self,
        issue_id: &str,
    ) -> Result<serde_json::Value> {
        let path = "/v0.2/payment/simulate";
        let query = format!("issueId={}", issue_id);
        let response = self.get(path, Some(&query)).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        
        Ok(json["data"].clone())
    }
}
```

- [ ] **Step 2: Run cargo check to verify methods compile**

Run: `cargo check`
Expected: Compiles successfully

- [ ] **Step 3: Commit**

```bash
git add src/client.rs
git commit -m "feat: add payment methods to ChimoneyClient"
```

## Notes

- This task adds payment-related methods to the ChimoneyClient
- Methods include: initiate_payment, verify_payment, simulate_payment
- All methods return typed responses using the types defined in Task 3
- simulate_payment is for sandbox testing only
- This builds on the foundation tasks (error types, types, client struct)
