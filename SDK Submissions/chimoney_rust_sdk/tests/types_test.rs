use chimoney_rust_sdk::types::*;
use chimoney_rust_sdk::ChimoneyError;

#[test]
fn test_transaction_serialization() {
    let transaction = Transaction {
        id: "txn_123".to_string(),
        amount: 100.0,
        currency: "USD".to_string(),
        status: "completed".to_string(),
        description: Some("Test payment".to_string()),
        created_at: Some("2024-01-01T00:00:00Z".to_string()),
    };

    let json = serde_json::to_string(&transaction).unwrap();
    assert!(json.contains("id"));
    assert!(json.contains("txn_123"));
    assert!(json.contains("amount"));
    assert!(json.contains("100.0"));
    assert!(json.contains("currency"));
    assert!(json.contains("USD"));
    assert!(json.contains("status"));
    assert!(json.contains("completed"));
    assert!(json.contains("description"));
    assert!(json.contains("Test payment"));
    assert!(json.contains("createdAt"));
}

#[test]
fn test_transaction_deserialization() {
    let json = r#"{
        "id": "txn_123",
        "amount": 100.0,
        "currency": "USD",
        "status": "completed",
        "description": "Test payment",
        "createdAt": "2024-01-01T00:00:00Z"
    }"#;

    let transaction: Transaction = serde_json::from_str(json).unwrap();
    assert_eq!(transaction.id, "txn_123");
    assert_eq!(transaction.amount, 100.0);
    assert_eq!(transaction.currency, "USD");
    assert_eq!(transaction.status, "completed");
    assert_eq!(transaction.description, Some("Test payment".to_string()));
    assert_eq!(
        transaction.created_at,
        Some("2024-01-01T00:00:00Z".to_string())
    );
}

#[test]
fn test_transfer_request_serialization() {
    let request = TransferRequest {
        receiver: "user@example.com".to_string(),
        value_in_usd: 50.0,
        sub_account: Some("sub_123".to_string()),
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("receiver"));
    assert!(json.contains("user@example.com"));
    assert!(json.contains("valueInUsd"));
    assert!(json.contains("50.0"));
    assert!(json.contains("subAccount"));
    assert!(json.contains("sub_123"));
}

#[test]
fn test_payment_request_serialization() {
    let request = PaymentRequest {
        email: "payer@example.com".to_string(),
        amount: 100.0,
        redirect_url: "https://example.com/callback".to_string(),
        sub_account: None,
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("email"));
    assert!(json.contains("payer@example.com"));
    assert!(json.contains("amount"));
    assert!(json.contains("100.0"));
    assert!(json.contains("redirectUrl"));
    assert!(json.contains("https://example.com/callback"));
}

#[test]
fn test_payment_response_deserialization() {
    let json = r#"{
        "id": "pay_123",
        "status": "pending",
        "checkoutUrl": "https://checkout.example.com/pay_123",
        "message": "Payment initiated"
    }"#;

    let response: PaymentResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.id, "pay_123");
    assert_eq!(response.status, "pending");
    assert_eq!(
        response.checkout_url,
        Some("https://checkout.example.com/pay_123".to_string())
    );
    assert_eq!(response.message, Some("Payment initiated".to_string()));
}

#[test]
fn test_bank_payout_request_serialization() {
    let request = BankPayoutRequest {
        base: PayoutRequest {
            sub_account: Some("sub_123".to_string()),
            turn_off_notification: Some(true),
        },
        transfers: vec![BankTransfer {
            bank_code: "044".to_string(),
            account_number: "1234567890".to_string(),
            amount: 100.0,
            currency: "NGN".to_string(),
            country_code: "NG".to_string(),
            beneficiary_name: Some("John Doe".to_string()),
        }],
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("subAccount"));
    assert!(json.contains("sub_123"));
    assert!(json.contains("turnOffNotification"));
    assert!(json.contains("true"));
    assert!(json.contains("transfers"));
    assert!(json.contains("bankCode"));
    assert!(json.contains("044"));
    assert!(json.contains("accountNumber"));
    assert!(json.contains("1234567890"));
    assert!(json.contains("amount"));
    assert!(json.contains("100.0"));
    assert!(json.contains("currency"));
    assert!(json.contains("NGN"));
    assert!(json.contains("countryCode"));
    assert!(json.contains("NG"));
    assert!(json.contains("beneficiaryName"));
    assert!(json.contains("John Doe"));
}

#[test]
fn test_redeem_airtime_request_serialization() {
    let request = RedeemAirtimeRequest {
        base: RedeemRequest {
            sub_account: "sub_123".to_string(),
            chi_ref: Some("ref_123".to_string()),
            turn_off_notification: None,
        },
        country_to_send: "NG".to_string(),
        phone_number: "+2348012345678".to_string(),
        test: Some(true),
    };

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("subAccount"));
    assert!(json.contains("sub_123"));
    assert!(json.contains("chiRef"));
    assert!(json.contains("ref_123"));
    assert!(json.contains("countryToSend"));
    assert!(json.contains("NG"));
    assert!(json.contains("phoneNumber"));
    assert!(json.contains("+2348012345678"));
    assert!(json.contains("test"));
    assert!(json.contains("true"));
}

#[test]
fn test_sub_account_serialization() {
    let sub_account = SubAccount {
        id: "sub_123".to_string(),
        name: "Test Account".to_string(),
        email: Some("test@example.com".to_string()),
        first_name: Some("John".to_string()),
        last_name: Some("Doe".to_string()),
        phone_number: Some("+1234567890".to_string()),
    };

    let json = serde_json::to_string(&sub_account).unwrap();
    assert!(json.contains("id"));
    assert!(json.contains("sub_123"));
    assert!(json.contains("name"));
    assert!(json.contains("Test Account"));
    assert!(json.contains("email"));
    assert!(json.contains("test@example.com"));
    assert!(json.contains("firstName"));
    assert!(json.contains("John"));
    assert!(json.contains("lastName"));
    assert!(json.contains("Doe"));
    assert!(json.contains("phoneNumber"));
    assert!(json.contains("+1234567890"));
}

#[test]
fn test_wallet_serialization() {
    let wallet = Wallet {
        id: "wallet_123".to_string(),
        name: Some("Main Wallet".to_string()),
        currency: Some("USD".to_string()),
        balance: Some(1000.0),
    };

    let json = serde_json::to_string(&wallet).unwrap();
    assert!(json.contains("id"));
    assert!(json.contains("wallet_123"));
    assert!(json.contains("name"));
    assert!(json.contains("Main Wallet"));
    assert!(json.contains("currency"));
    assert!(json.contains("USD"));
    assert!(json.contains("balance"));
    assert!(json.contains("1000.0"));
}

#[test]
fn test_error_type() {
    let error = ChimoneyError::ApiKeyEmpty;
    assert!(!error.to_string().is_empty());

    let error = ChimoneyError::ApiError {
        status: 400,
        message: "Bad request".to_string(),
    };
    assert!(error.to_string().contains("400"));
    assert!(error.to_string().contains("Bad request"));

    let error = ChimoneyError::RateLimited { retry_after: 60 };
    assert!(error.to_string().contains("60"));
}
