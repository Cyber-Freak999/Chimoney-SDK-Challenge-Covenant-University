use chimoney_rust_sdk::types::*;
use chimoney_rust_sdk::ChimoneyError;
use std::collections::HashMap;

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

// === Account types ===

#[test]
fn test_transfer_response_deserialization() {
    let json = r#"{
        "id": "txn_456",
        "status": "success",
        "message": "Transfer completed"
    }"#;
    let response: TransferResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.id, "txn_456");
    assert_eq!(response.status, "success");
    assert_eq!(response.message, Some("Transfer completed".to_string()));
}

#[test]
fn test_initiate_chimoney_request_serialization() {
    let request = InitiateChimoneyRequest {
        receiver: "user@example.com".to_string(),
        value_in_usd: 25.0,
        sub_account: None,
        turn_off_notification: Some(true),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("receiver"));
    assert!(json.contains("user@example.com"));
    assert!(json.contains("valueInUsd"));
    assert!(json.contains("25.0"));
    assert!(json.contains("turnOffNotification"));
    assert!(json.contains("true"));
    assert!(!json.contains("subAccount"));
}

#[test]
fn test_initiate_chimoney_response_deserialization() {
    let json = r#"{
        "id": "chi_789",
        "status": "pending",
        "message": "Transaction initiated"
    }"#;
    let response: InitiateChimoneyResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.id, "chi_789");
    assert_eq!(response.status, "pending");
    assert_eq!(response.message, Some("Transaction initiated".to_string()));
}

#[test]
fn test_delete_unpaid_transaction_response_deserialization() {
    let json = r#"{
        "status": "success",
        "message": "Transactions deleted"
    }"#;
    let response: DeleteUnpaidTransactionResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, Some("Transactions deleted".to_string()));
}

// === Payment types ===

#[test]
fn test_payment_verification_deserialization() {
    let json = r#"{
        "id": "pay_ver_001",
        "status": "verified",
        "amount": 150.50,
        "currency": "USD",
        "message": "Payment verified"
    }"#;
    let verification: PaymentVerification = serde_json::from_str(json).unwrap();
    assert_eq!(verification.id, "pay_ver_001");
    assert_eq!(verification.status, "verified");
    assert_eq!(verification.amount, Some(150.50));
    assert_eq!(verification.currency, Some("USD".to_string()));
    assert_eq!(verification.message, Some("Payment verified".to_string()));
}

// === Payout types ===

#[test]
fn test_payout_request_base_serialization() {
    let request = PayoutRequest {
        sub_account: None,
        turn_off_notification: None,
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(!json.contains("subAccount"));
    assert!(!json.contains("turnOffNotification"));
}

#[test]
fn test_airtime_payout_request_serialization() {
    let request = AirtimePayoutRequest {
        base: PayoutRequest {
            sub_account: Some("sub_001".to_string()),
            turn_off_notification: None,
        },
        transfers: vec![AirtimeTransfer {
            phone_number: "+2348012345678".to_string(),
            amount: 10.0,
            country_code: "NG".to_string(),
        }],
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("subAccount"));
    assert!(json.contains("sub_001"));
    assert!(json.contains("transfers"));
    assert!(json.contains("phoneNumber"));
    assert!(json.contains("+2348012345678"));
    assert!(json.contains("amount"));
    assert!(json.contains("10.0"));
    assert!(json.contains("countryCode"));
    assert!(json.contains("NG"));
    assert!(!json.contains("turnOffNotification"));
}

#[test]
fn test_chimoney_payout_request_serialization() {
    let request = ChimoneyPayoutRequest {
        base: PayoutRequest {
            sub_account: None,
            turn_off_notification: Some(false),
        },
        transfers: vec![ChimoneyTransfer {
            receiver: "user@test.com".to_string(),
            value_in_usd: 5.0,
        }],
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("turnOffNotification"));
    assert!(json.contains("false"));
    assert!(json.contains("transfers"));
    assert!(json.contains("receiver"));
    assert!(json.contains("user@test.com"));
    assert!(json.contains("valueInUsd"));
    assert!(json.contains("5.0"));
    assert!(!json.contains("subAccount"));
}

#[test]
fn test_mobile_money_payout_request_serialization() {
    let request = MobileMoneyPayoutRequest {
        base: PayoutRequest {
            sub_account: None,
            turn_off_notification: None,
        },
        transfers: vec![MobileMoneyTransfer {
            phone_number: "+233201234567".to_string(),
            amount: 20.0,
            country_code: "GH".to_string(),
            provider_code: "MTN".to_string(),
        }],
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("transfers"));
    assert!(json.contains("phoneNumber"));
    assert!(json.contains("+233201234567"));
    assert!(json.contains("amount"));
    assert!(json.contains("20.0"));
    assert!(json.contains("countryCode"));
    assert!(json.contains("GH"));
    assert!(json.contains("providerCode"));
    assert!(json.contains("MTN"));
}

#[test]
fn test_giftcard_payout_request_serialization() {
    let request = GiftCardPayoutRequest {
        base: PayoutRequest {
            sub_account: Some("sub_gc".to_string()),
            turn_off_notification: Some(true),
        },
        transfers: vec![GiftCardTransfer {
            receiver: "gift@example.com".to_string(),
            value_in_usd: 50.0,
            provider: "amazon".to_string(),
        }],
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("subAccount"));
    assert!(json.contains("sub_gc"));
    assert!(json.contains("turnOffNotification"));
    assert!(json.contains("transfers"));
    assert!(json.contains("receiver"));
    assert!(json.contains("gift@example.com"));
    assert!(json.contains("valueInUsd"));
    assert!(json.contains("50.0"));
    assert!(json.contains("provider"));
    assert!(json.contains("amazon"));
}

#[test]
fn test_interledger_payout_request_serialization() {
    let request = InterledgerPayoutRequest {
        base: PayoutRequest {
            sub_account: None,
            turn_off_notification: None,
        },
        transfers: vec![InterledgerTransfer {
            receiver_address: "ilp_addr_123".to_string(),
            value_in_usd: 30.0,
        }],
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("transfers"));
    assert!(json.contains("receiverAddress"));
    assert!(json.contains("ilp_addr_123"));
    assert!(json.contains("valueInUsd"));
    assert!(json.contains("30.0"));
}

#[test]
fn test_wallet_payout_request_serialization() {
    let request = WalletPayoutRequest {
        base: PayoutRequest {
            sub_account: Some("sub_w".to_string()),
            turn_off_notification: Some(true),
        },
        transfers: vec![WalletTransfer {
            receiver: "wallet_user@test.com".to_string(),
            value_in_usd: 40.0,
            wallet_id: "wallet_abc".to_string(),
        }],
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("subAccount"));
    assert!(json.contains("sub_w"));
    assert!(json.contains("turnOffNotification"));
    assert!(json.contains("transfers"));
    assert!(json.contains("receiver"));
    assert!(json.contains("wallet_user@test.com"));
    assert!(json.contains("valueInUsd"));
    assert!(json.contains("40.0"));
    assert!(json.contains("walletId"));
    assert!(json.contains("wallet_abc"));
}

#[test]
fn test_payout_response_deserialization() {
    let json = r#"{
        "status": "success",
        "message": "Payout processed",
        "id": "payout_001"
    }"#;
    let response: PayoutResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, Some("Payout processed".to_string()));
    assert_eq!(response.id, Some("payout_001".to_string()));
}

#[test]
fn test_payout_status_response_deserialization() {
    let json = r#"{
        "status": "success",
        "message": "Status retrieved",
        "data": {"id": "payout_001", "status": "completed"}
    }"#;
    let response: PayoutStatusResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, Some("Status retrieved".to_string()));
    assert!(response.data.is_some());
    let data = response.data.unwrap();
    assert_eq!(data["id"], "payout_001");
    assert_eq!(data["status"], "completed");
}

// === Redeem types ===

#[test]
fn test_redeem_request_base_serialization() {
    let request = RedeemRequest {
        sub_account: "sub_redeem".to_string(),
        chi_ref: None,
        turn_off_notification: None,
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("subAccount"));
    assert!(json.contains("sub_redeem"));
    assert!(!json.contains("chiRef"));
    assert!(!json.contains("turnOffNotification"));
}

#[test]
fn test_redeem_chimoney_request_serialization() {
    let mut chimoneys = HashMap::new();
    chimoneys.insert("tx_1".to_string(), "10.0".to_string());
    chimoneys.insert("tx_2".to_string(), "20.0".to_string());
    let request = RedeemChimoneyRequest {
        base: RedeemRequest {
            sub_account: "sub_rc".to_string(),
            chi_ref: Some("ref_chi".to_string()),
            turn_off_notification: Some(true),
        },
        chimoneys,
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("subAccount"));
    assert!(json.contains("sub_rc"));
    assert!(json.contains("chiRef"));
    assert!(json.contains("ref_chi"));
    assert!(json.contains("turnOffNotification"));
    assert!(json.contains("chimoneys"));
    assert!(json.contains("tx_1"));
    assert!(json.contains("tx_2"));
}

#[test]
fn test_redeem_giftcard_request_serialization() {
    let mut redeem_options = HashMap::new();
    redeem_options.insert("gc_1".to_string(), "50.0".to_string());
    let request = RedeemGiftCardRequest {
        base: RedeemRequest {
            sub_account: "sub_gc_redeem".to_string(),
            chi_ref: None,
            turn_off_notification: None,
        },
        redeem_options,
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("subAccount"));
    assert!(json.contains("sub_gc_redeem"));
    assert!(json.contains("redeemOptions"));
    assert!(json.contains("gc_1"));
    assert!(!json.contains("chiRef"));
    assert!(!json.contains("turnOffNotification"));
}

#[test]
fn test_redeem_mobile_money_request_serialization() {
    let mut redeem_options = HashMap::new();
    redeem_options.insert("mm_1".to_string(), "30.0".to_string());
    let request = RedeemMobileMoneyRequest {
        base: RedeemRequest {
            sub_account: "sub_mm".to_string(),
            chi_ref: Some("ref_mm".to_string()),
            turn_off_notification: None,
        },
        redeem_options,
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("subAccount"));
    assert!(json.contains("sub_mm"));
    assert!(json.contains("chiRef"));
    assert!(json.contains("ref_mm"));
    assert!(json.contains("redeemOptions"));
    assert!(json.contains("mm_1"));
}

#[test]
fn test_redeem_response_deserialization() {
    let json = r#"{
        "status": "success",
        "message": "Redeem processed",
        "id": "redeem_001"
    }"#;
    let response: RedeemResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, Some("Redeem processed".to_string()));
    assert_eq!(response.id, Some("redeem_001".to_string()));
}

// === SubAccount types ===

#[test]
fn test_sub_account_deserialization() {
    let json = r#"{
        "id": "sub_abc",
        "name": "Test Sub",
        "email": "sub@test.com",
        "firstName": "Jane",
        "lastName": "Smith",
        "phoneNumber": "+15551234567"
    }"#;
    let sub: SubAccount = serde_json::from_str(json).unwrap();
    assert_eq!(sub.id, "sub_abc");
    assert_eq!(sub.name, "Test Sub");
    assert_eq!(sub.email, Some("sub@test.com".to_string()));
    assert_eq!(sub.first_name, Some("Jane".to_string()));
    assert_eq!(sub.last_name, Some("Smith".to_string()));
    assert_eq!(sub.phone_number, Some("+15551234567".to_string()));
}

#[test]
fn test_create_sub_account_request_serialization() {
    let request = CreateSubAccountRequest {
        name: "New Sub".to_string(),
        first_name: "Alice".to_string(),
        last_name: "Wonder".to_string(),
        email: "alice@sub.com".to_string(),
        phone_number: "+15559876543".to_string(),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("name"));
    assert!(json.contains("New Sub"));
    assert!(json.contains("firstName"));
    assert!(json.contains("Alice"));
    assert!(json.contains("lastName"));
    assert!(json.contains("Wonder"));
    assert!(json.contains("email"));
    assert!(json.contains("alice@sub.com"));
    assert!(json.contains("phoneNumber"));
    assert!(json.contains("+15559876543"));
}

#[test]
fn test_update_sub_account_request_all_none_serialization() {
    let request = UpdateSubAccountRequest {
        id: "sub_upd".to_string(),
        first_name: None,
        last_name: None,
        phone_number: None,
        meta: None,
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("id"));
    assert!(json.contains("sub_upd"));
    assert!(!json.contains("firstName"));
    assert!(!json.contains("lastName"));
    assert!(!json.contains("phoneNumber"));
    assert!(!json.contains("meta"));
}

#[test]
fn test_sub_account_response_deserialization() {
    let json = r#"{
        "status": "success",
        "message": "Sub-account created",
        "data": {"id": "sub_new", "name": "New Sub"}
    }"#;
    let response: SubAccountResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, Some("Sub-account created".to_string()));
    assert!(response.data.is_some());
    let data = response.data.unwrap();
    assert_eq!(data["id"], "sub_new");
    assert_eq!(data["name"], "New Sub");
}

#[test]
fn test_sub_account_list_deserialization() {
    let json = r#"{
        "status": "success",
        "data": []
    }"#;
    let list: SubAccountList = serde_json::from_str(json).unwrap();
    assert_eq!(list.status, "success");
    assert!(list.data.is_some());
    assert!(list.data.unwrap().is_empty());
}

// === Wallet types ===

#[test]
fn test_wallet_deserialization() {
    let json = r#"{
        "id": "wall_001",
        "name": "Savings",
        "currency": "USD",
        "balance": 500.75
    }"#;
    let wallet: Wallet = serde_json::from_str(json).unwrap();
    assert_eq!(wallet.id, "wall_001");
    assert_eq!(wallet.name, Some("Savings".to_string()));
    assert_eq!(wallet.currency, Some("USD".to_string()));
    assert_eq!(wallet.balance, Some(500.75));
}

#[test]
fn test_wallet_list_deserialization() {
    let json = r#"{
        "status": "success",
        "data": []
    }"#;
    let list: WalletList = serde_json::from_str(json).unwrap();
    assert_eq!(list.status, "success");
    assert!(list.data.is_some());
    assert!(list.data.unwrap().is_empty());
}

#[test]
fn test_wallet_lookup_request_serialization() {
    let request = WalletLookupRequest {
        wallet_id: "wall_lk".to_string(),
        sub_account: "sub_lk".to_string(),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("walletId"));
    assert!(json.contains("wall_lk"));
    assert!(json.contains("subAccount"));
    assert!(json.contains("sub_lk"));
}

#[test]
fn test_wallet_transfer_request_serialization() {
    let request = WalletTransferRequest {
        wallet: "wall_src".to_string(),
        value_in_usd: 75.0,
        sub_account: "sub_wt".to_string(),
        receiver: "recv@test.com".to_string(),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("wallet"));
    assert!(json.contains("wall_src"));
    assert!(json.contains("valueInUsd"));
    assert!(json.contains("75.0"));
    assert!(json.contains("subAccount"));
    assert!(json.contains("sub_wt"));
    assert!(json.contains("receiver"));
    assert!(json.contains("recv@test.com"));
}

#[test]
fn test_wallet_response_deserialization() {
    let json = r#"{
        "status": "success",
        "message": "Wallet retrieved",
        "data": {"id": "wall_001"}
    }"#;
    let response: WalletResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, Some("Wallet retrieved".to_string()));
    assert!(response.data.is_some());
}

// === Edge case tests ===

#[test]
fn test_transaction_all_fields_none() {
    let transaction = Transaction {
        id: "txn_edge".to_string(),
        amount: 5.0,
        currency: "USD".to_string(),
        status: "pending".to_string(),
        description: None,
        created_at: None,
    };
    let json = serde_json::to_string(&transaction).unwrap();
    assert!(json.contains("id"));
    assert!(json.contains("txn_edge"));
    assert!(json.contains("description"));
    assert!(json.contains("createdAt"));
    assert!(!json.contains("Test payment"));
}

#[test]
fn test_payment_request_with_sub_account() {
    let request = PaymentRequest {
        email: "payer@test.com".to_string(),
        amount: 200.0,
        redirect_url: "https://example.com/ok".to_string(),
        sub_account: Some("sub_pa".to_string()),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("subAccount"));
    assert!(json.contains("sub_pa"));
    assert!(json.contains("email"));
    assert!(json.contains("payer@test.com"));
}

#[test]
fn test_bank_transfer_optional_beneficiary_name() {
    let transfer = BankTransfer {
        bank_code: "011".to_string(),
        account_number: "9876543210".to_string(),
        amount: 200.0,
        currency: "NGN".to_string(),
        country_code: "NG".to_string(),
        beneficiary_name: None,
    };
    let json = serde_json::to_string(&transfer).unwrap();
    assert!(json.contains("bankCode"));
    assert!(json.contains("011"));
    assert!(json.contains("accountNumber"));
    assert!(json.contains("9876543210"));
    assert!(!json.contains("beneficiaryName"));
}

#[test]
fn test_update_sub_account_request_all_some() {
    let mut meta = HashMap::new();
    meta.insert("key1".to_string(), "val1".to_string());
    let request = UpdateSubAccountRequest {
        id: "sub_all".to_string(),
        first_name: Some("Bob".to_string()),
        last_name: Some("Builder".to_string()),
        phone_number: Some("+15550001111".to_string()),
        meta: Some(meta),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("id"));
    assert!(json.contains("sub_all"));
    assert!(json.contains("firstName"));
    assert!(json.contains("Bob"));
    assert!(json.contains("lastName"));
    assert!(json.contains("Builder"));
    assert!(json.contains("phoneNumber"));
    assert!(json.contains("+15550001111"));
    assert!(json.contains("meta"));
    assert!(json.contains("key1"));
    assert!(json.contains("val1"));
}

// === Agent types ===

#[test]
fn test_agent_deserialization() {
    let json = r#"{
        "id": "agent_001",
        "name": "Test Agent",
        "description": "A test agent",
        "status": "active",
        "walletId": "wallet_abc",
        "passport": {"type": "national_id"},
        "meta": {"role": "assistant"},
        "createdAt": "2024-01-01T00:00:00Z"
    }"#;
    let agent: Agent = serde_json::from_str(json).unwrap();
    assert_eq!(agent.id, "agent_001");
    assert_eq!(agent.name, "Test Agent");
    assert_eq!(agent.description, Some("A test agent".to_string()));
    assert_eq!(agent.status, "active");
    assert_eq!(agent.wallet_id, Some("wallet_abc".to_string()));
    assert!(agent.passport.is_some());
    assert!(agent.meta.is_some());
    assert_eq!(agent.created_at, Some("2024-01-01T00:00:00Z".to_string()));
}

#[test]
fn test_agent_serialization() {
    let agent = Agent {
        id: "agent_002".to_string(),
        name: "Agent Two".to_string(),
        description: None,
        status: "active".to_string(),
        wallet_id: None,
        passport: None,
        meta: None,
        created_at: None,
    };
    let json = serde_json::to_string(&agent).unwrap();
    assert!(json.contains("id"));
    assert!(json.contains("agent_002"));
    assert!(json.contains("name"));
    assert!(json.contains("Agent Two"));
    assert!(json.contains("status"));
    assert!(json.contains("active"));
}

#[test]
fn test_create_agent_request_serialization() {
    let request = CreateAgentRequest {
        name: "My Agent".to_string(),
        description: Some("An AI agent".to_string()),
        meta: Some(serde_json::json!({"key": "value"})),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("name"));
    assert!(json.contains("My Agent"));
    assert!(json.contains("description"));
    assert!(json.contains("An AI agent"));
    assert!(json.contains("meta"));
}

#[test]
fn test_create_agent_request_optional_fields() {
    let request = CreateAgentRequest {
        name: "Minimal Agent".to_string(),
        description: None,
        meta: None,
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("name"));
    assert!(json.contains("Minimal Agent"));
    assert!(!json.contains("description"));
    assert!(!json.contains("meta"));
}

#[test]
fn test_update_agent_request_serialization() {
    let request = UpdateAgentRequest {
        agent_id: "agent_upd".to_string(),
        name: Some("Updated Name".to_string()),
        description: None,
        meta: None,
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("agentId"));
    assert!(json.contains("agent_upd"));
    assert!(json.contains("name"));
    assert!(json.contains("Updated Name"));
    assert!(!json.contains("description"));
}

#[test]
fn test_update_agent_policies_request_serialization() {
    let request = UpdateAgentPoliciesRequest {
        agent_id: "agent_pol".to_string(),
        limits: Some(serde_json::json!({"maxTxn": 100})),
        capabilities: None,
        regions: Some(serde_json::json!({"allow": ["US", "NG"]})),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("agentId"));
    assert!(json.contains("agent_pol"));
    assert!(json.contains("limits"));
    assert!(json.contains("maxTxn"));
    assert!(!json.contains("capabilities"));
    assert!(json.contains("regions"));
}

#[test]
fn test_agent_id_request_serialization() {
    let request = AgentIdRequest {
        agent_id: "agent_suspend".to_string(),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("agentId"));
    assert!(json.contains("agent_suspend"));
}

#[test]
fn test_agent_api_key_response_deserialization() {
    let json = r#"{
        "agentId": "agent_key",
        "apiKeyPrefix": "chm_abc123",
        "createdAt": "2024-06-01T00:00:00Z"
    }"#;
    let response: AgentApiKeyResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.agent_id, Some("agent_key".to_string()));
    assert_eq!(response.api_key_prefix, Some("chm_abc123".to_string()));
    assert_eq!(response.created_at, Some("2024-06-01T00:00:00Z".to_string()));
}

#[test]
fn test_agent_api_key_response_empty() {
    let json = r#"{}"#;
    let response: AgentApiKeyResponse = serde_json::from_str(json).unwrap();
    assert!(response.agent_id.is_none());
    assert!(response.api_key_prefix.is_none());
    assert!(response.created_at.is_none());
}

#[test]
fn test_manage_agent_api_key_request_serialization() {
    let request = ManageAgentApiKeyRequest {
        agent_id: "agent_manage".to_string(),
        action: "rotate".to_string(),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("agentId"));
    assert!(json.contains("agent_manage"));
    assert!(json.contains("action"));
    assert!(json.contains("rotate"));
}

#[test]
fn test_agent_transactions_request_serialization() {
    let request = AgentTransactionsRequest {
        agent_id: "agent_txn".to_string(),
        sub_account: Some("sub_123".to_string()),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("agentId"));
    assert!(json.contains("agent_txn"));
    assert!(json.contains("subAccount"));
    assert!(json.contains("sub_123"));
}

#[test]
fn test_agent_transactions_request_no_sub_account() {
    let request = AgentTransactionsRequest {
        agent_id: "agent_txn2".to_string(),
        sub_account: None,
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("agentId"));
    assert!(json.contains("agent_txn2"));
    assert!(!json.contains("subAccount"));
}

#[test]
fn test_agent_transactions_response_deserialization() {
    let json = r#"{
        "status": "success",
        "data": [{"id": "txn_1", "amount": 10.0}]
    }"#;
    let response: AgentTransactionsResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert!(response.data.is_some());
    let data = response.data.unwrap();
    assert_eq!(data.len(), 1);
}

#[test]
fn test_agent_transactions_response_empty_data() {
    let json = r#"{
        "status": "success",
        "data": []
    }"#;
    let response: AgentTransactionsResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert!(response.data.is_some());
    assert!(response.data.unwrap().is_empty());
}

#[test]
fn test_agent_response_deserialization() {
    let json = r#"{
        "status": "success",
        "data": {
            "id": "agent_resp",
            "name": "Resp Agent",
            "description": null,
            "status": "active",
            "walletId": null,
            "passport": null,
            "meta": null,
            "createdAt": "2024-01-01T00:00:00Z"
        }
    }"#;
    let response: AgentResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert!(response.data.is_some());
    let agent = response.data.unwrap();
    assert_eq!(agent.id, "agent_resp");
    assert_eq!(agent.name, "Resp Agent");
    assert_eq!(agent.status, "active");
}

#[test]
fn test_agent_response_no_data() {
    let json = r#"{
        "status": "success"
    }"#;
    let response: AgentResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert!(response.data.is_none());
}

#[test]
fn test_agent_list_response_deserialization() {
    let json = r#"{
        "status": "success",
        "data": [
            {"id": "a1", "name": "Agent 1", "status": "active"},
            {"id": "a2", "name": "Agent 2", "status": "suspended"}
        ]
    }"#;
    let response: AgentListResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert!(response.data.is_some());
    let agents = response.data.unwrap();
    assert_eq!(agents.len(), 2);
    assert_eq!(agents[0].id, "a1");
    assert_eq!(agents[1].id, "a2");
}

#[test]
fn test_agent_list_response_empty() {
    let json = r#"{
        "status": "success",
        "data": []
    }"#;
    let response: AgentListResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert!(response.data.is_some());
    assert!(response.data.unwrap().is_empty());
}

#[test]
fn test_fund_agent_request_serialization() {
    let request = FundAgentRequest {
        agent_id: "agent_fund".to_string(),
        amount_in_usd: 25.50,
        sub_account: Some("sub_fund".to_string()),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("agentId"));
    assert!(json.contains("agent_fund"));
    assert!(json.contains("amountInUsd"));
    assert!(json.contains("25.5"));
    assert!(json.contains("subAccount"));
    assert!(json.contains("sub_fund"));
}

#[test]
fn test_fund_agent_request_no_sub_account() {
    let request = FundAgentRequest {
        agent_id: "agent_fund2".to_string(),
        amount_in_usd: 100.0,
        sub_account: None,
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("agentId"));
    assert!(json.contains("agent_fund2"));
    assert!(json.contains("amountInUsd"));
    assert!(json.contains("100.0"));
    assert!(!json.contains("subAccount"));
}

// === Beneficiary types ===

#[test]
fn test_beneficiary_deserialization() {
    let json = r#"{
        "id": "ben_001",
        "beneficiaryType": "bank",
        "accountNumber": "1234567890",
        "bankCode": "044",
        "countryCode": "NG",
        "name": "John Doe",
        "createdAt": "2024-01-01T00:00:00Z"
    }"#;
    let beneficiary: Beneficiary = serde_json::from_str(json).unwrap();
    assert_eq!(beneficiary.id, "ben_001");
    assert_eq!(beneficiary.beneficiary_type, Some("bank".to_string()));
    assert_eq!(beneficiary.account_number, Some("1234567890".to_string()));
    assert_eq!(beneficiary.bank_code, Some("044".to_string()));
    assert_eq!(beneficiary.country_code, Some("NG".to_string()));
    assert_eq!(beneficiary.name, "John Doe");
    assert_eq!(beneficiary.created_at, Some("2024-01-01T00:00:00Z".to_string()));
}

#[test]
fn test_beneficiary_minimal_fields() {
    let json = r#"{
        "id": "ben_min",
        "name": "Minimal Ben"
    }"#;
    let beneficiary: Beneficiary = serde_json::from_str(json).unwrap();
    assert_eq!(beneficiary.id, "ben_min");
    assert_eq!(beneficiary.name, "Minimal Ben");
    assert!(beneficiary.beneficiary_type.is_none());
    assert!(beneficiary.account_number.is_none());
    assert!(beneficiary.bank_code.is_none());
    assert!(beneficiary.country_code.is_none());
    assert!(beneficiary.created_at.is_none());
}

#[test]
fn test_beneficiary_serialization() {
    let beneficiary = Beneficiary {
        id: "ben_s".to_string(),
        beneficiary_type: Some("bank".to_string()),
        account_number: None,
        bank_code: Some("011".to_string()),
        country_code: None,
        name: "Serialize Me".to_string(),
        created_at: None,
    };
    let json = serde_json::to_string(&beneficiary).unwrap();
    assert!(json.contains("id"));
    assert!(json.contains("ben_s"));
    assert!(json.contains("beneficiaryType"));
    assert!(json.contains("bank"));
    assert!(json.contains("name"));
    assert!(json.contains("Serialize Me"));
    assert!(json.contains("bankCode"));
    assert!(json.contains("011"));
    assert!(!json.contains("accountNumber"));
    assert!(!json.contains("countryCode"));
    assert!(!json.contains("createdAt"));
}

#[test]
fn test_create_bank_beneficiary_request_serialization() {
    let request = CreateBankBeneficiaryRequest {
        account_number: "1234567890".to_string(),
        bank_code: "044".to_string(),
        country_code: "NG".to_string(),
        name: "Jane Doe".to_string(),
        currency: "NGN".to_string(),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("accountNumber"));
    assert!(json.contains("1234567890"));
    assert!(json.contains("bankCode"));
    assert!(json.contains("044"));
    assert!(json.contains("countryCode"));
    assert!(json.contains("NG"));
    assert!(json.contains("name"));
    assert!(json.contains("Jane Doe"));
    assert!(json.contains("currency"));
    assert!(json.contains("NGN"));
}

#[test]
fn test_beneficiary_response_deserialization() {
    let json = r#"{
        "status": "success",
        "message": "Beneficiary created",
        "data": {
            "id": "ben_resp",
            "beneficiaryType": "bank",
            "accountNumber": "9876543210",
            "bankCode": "011",
            "countryCode": "NG",
            "name": "Created Ben",
            "createdAt": "2024-06-01T00:00:00Z"
        }
    }"#;
    let response: BeneficiaryResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, Some("Beneficiary created".to_string()));
    let data = response.data.unwrap();
    assert_eq!(data.id, "ben_resp");
    assert_eq!(data.beneficiary_type, Some("bank".to_string()));
    assert_eq!(data.account_number, Some("9876543210".to_string()));
    assert_eq!(data.name, "Created Ben");
}

#[test]
fn test_beneficiary_response_no_data() {
    let json = r#"{
        "status": "success"
    }"#;
    let response: BeneficiaryResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert!(response.message.is_none());
    assert!(response.data.is_none());
}

#[test]
fn test_beneficiary_list_response_deserialization() {
    let json = r#"{
        "status": "success",
        "data": [
            {"id": "b1", "name": "Ben 1", "beneficiaryType": "bank"},
            {"id": "b2", "name": "Ben 2", "beneficiaryType": "airtime"}
        ]
    }"#;
    let response: BeneficiaryListResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    let data = response.data.unwrap();
    assert_eq!(data.len(), 2);
    assert_eq!(data[0].id, "b1");
    assert_eq!(data[0].name, "Ben 1");
    assert_eq!(data[1].id, "b2");
    assert_eq!(data[1].beneficiary_type, Some("airtime".to_string()));
}

#[test]
fn test_beneficiary_list_response_empty() {
    let json = r#"{
        "status": "success",
        "data": []
    }"#;
    let response: BeneficiaryListResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert!(response.data.is_some());
    assert!(response.data.unwrap().is_empty());
}

#[test]
fn test_beneficiary_list_response_no_data() {
    let json = r#"{
        "status": "success"
    }"#;
    let response: BeneficiaryListResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert!(response.data.is_none());
}

#[test]
fn test_preview_transfer_request_serialization() {
    let request = PreviewTransferRequest {
        beneficiary_id: "ben_prev".to_string(),
        amount: 150.0,
        currency: "USD".to_string(),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("beneficiaryId"));
    assert!(json.contains("ben_prev"));
    assert!(json.contains("amount"));
    assert!(json.contains("150.0"));
    assert!(json.contains("currency"));
    assert!(json.contains("USD"));
}

#[test]
fn test_preview_transfer_response_deserialization() {
    let json = r#"{
        "status": "success",
        "fee": 2.5,
        "exchangeRate": 1.2,
        "totalAmount": 152.5,
        "destinationAmount": 127.083
    }"#;
    let response: PreviewTransferResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.fee, Some(2.5));
    assert_eq!(response.exchange_rate, Some(1.2));
    assert_eq!(response.total_amount, Some(152.5));
    assert_eq!(response.destination_amount, Some(127.083));
}

#[test]
fn test_preview_transfer_response_optional_fields() {
    let json = r#"{
        "status": "success"
    }"#;
    let response: PreviewTransferResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert!(response.fee.is_none());
    assert!(response.exchange_rate.is_none());
    assert!(response.total_amount.is_none());
    assert!(response.destination_amount.is_none());
}

// === Multicurrency Wallet types ===

#[test]
fn test_multicurrency_wallet_deserialization() {
    let json = r#"{
        "id": "mcw_001",
        "currency": "EUR",
        "balance": 1250.50,
        "subAccount": "sub_mc",
        "meta": {"label": "EUR Wallet"},
        "createdAt": "2024-01-01T00:00:00Z"
    }"#;
    let wallet: MulticurrencyWallet = serde_json::from_str(json).unwrap();
    assert_eq!(wallet.id, "mcw_001");
    assert_eq!(wallet.currency, "EUR");
    assert_eq!(wallet.balance, Some(1250.50));
    assert_eq!(wallet.sub_account, Some("sub_mc".to_string()));
    assert!(wallet.meta.is_some());
    assert_eq!(wallet.created_at, Some("2024-01-01T00:00:00Z".to_string()));
}

#[test]
fn test_multicurrency_wallet_minimal() {
    let json = r#"{
        "id": "mcw_min",
        "currency": "GBP"
    }"#;
    let wallet: MulticurrencyWallet = serde_json::from_str(json).unwrap();
    assert_eq!(wallet.id, "mcw_min");
    assert_eq!(wallet.currency, "GBP");
    assert!(wallet.balance.is_none());
    assert!(wallet.sub_account.is_none());
    assert!(wallet.meta.is_none());
    assert!(wallet.created_at.is_none());
}

#[test]
fn test_multicurrency_wallet_serialization() {
    let wallet = MulticurrencyWallet {
        id: "mcw_s".to_string(),
        currency: "NGN".to_string(),
        balance: Some(500.0),
        sub_account: Some("sub_s".to_string()),
        meta: None,
        created_at: None,
    };
    let json = serde_json::to_string(&wallet).unwrap();
    assert!(json.contains("id"));
    assert!(json.contains("mcw_s"));
    assert!(json.contains("currency"));
    assert!(json.contains("NGN"));
    assert!(json.contains("balance"));
    assert!(json.contains("subAccount"));
    assert!(json.contains("sub_s"));
}

#[test]
fn test_create_multicurrency_wallet_request_serialization() {
    let request = CreateMulticurrencyWalletRequest {
        sub_account: "sub_create".to_string(),
        currency: "EUR".to_string(),
        meta: Some(serde_json::json!({"purpose": "savings"})),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("subAccount"));
    assert!(json.contains("sub_create"));
    assert!(json.contains("currency"));
    assert!(json.contains("EUR"));
    assert!(json.contains("meta"));
    assert!(json.contains("purpose"));
}

#[test]
fn test_create_multicurrency_wallet_request_no_meta() {
    let request = CreateMulticurrencyWalletRequest {
        sub_account: "sub_nm".to_string(),
        currency: "USD".to_string(),
        meta: None,
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("subAccount"));
    assert!(json.contains("sub_nm"));
    assert!(json.contains("currency"));
    assert!(json.contains("USD"));
    assert!(!json.contains("meta"));
}

#[test]
fn test_update_multicurrency_wallet_request_serialization() {
    let request = UpdateMulticurrencyWalletRequest {
        wallet_id: "mcw_upd".to_string(),
        meta: Some(serde_json::json!({"label": "updated"})),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("walletId"));
    assert!(json.contains("mcw_upd"));
    assert!(json.contains("meta"));
    assert!(json.contains("updated"));
}

#[test]
fn test_update_multicurrency_wallet_request_no_meta() {
    let request = UpdateMulticurrencyWalletRequest {
        wallet_id: "mcw_up2".to_string(),
        meta: None,
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("walletId"));
    assert!(json.contains("mcw_up2"));
    assert!(!json.contains("meta"));
}

#[test]
fn test_multicurrency_wallet_response_deserialization() {
    let json = r#"{
        "status": "success",
        "message": "Wallet created",
        "data": {
            "id": "mcw_resp",
            "currency": "EUR",
            "balance": 0.0,
            "subAccount": "sub_resp",
            "createdAt": "2024-06-01T00:00:00Z"
        }
    }"#;
    let response: MulticurrencyWalletResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, Some("Wallet created".to_string()));
    let data = response.data.unwrap();
    assert_eq!(data.id, "mcw_resp");
    assert_eq!(data.currency, "EUR");
}

#[test]
fn test_multicurrency_wallet_response_no_data() {
    let json = r#"{
        "status": "success"
    }"#;
    let response: MulticurrencyWalletResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert!(response.message.is_none());
    assert!(response.data.is_none());
}

#[test]
fn test_multicurrency_wallet_list_response_deserialization() {
    let json = r#"{
        "status": "success",
        "data": [
            {"id": "mcw1", "currency": "EUR"},
            {"id": "mcw2", "currency": "GBP"}
        ]
    }"#;
    let response: MulticurrencyWalletListResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    let data = response.data.unwrap();
    assert_eq!(data.len(), 2);
    assert_eq!(data[0].id, "mcw1");
    assert_eq!(data[1].currency, "GBP");
}

#[test]
fn test_multicurrency_wallet_list_response_empty() {
    let json = r#"{
        "status": "success",
        "data": []
    }"#;
    let response: MulticurrencyWalletListResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert!(response.data.is_some());
    assert!(response.data.unwrap().is_empty());
}

#[test]
fn test_multicurrency_wallet_list_response_no_data() {
    let json = r#"{
        "status": "success"
    }"#;
    let response: MulticurrencyWalletListResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert!(response.data.is_none());
}

#[test]
fn test_transfer_quote_request_serialization() {
    let request = TransferQuoteRequest {
        from_wallet: "mcw_from".to_string(),
        to_wallet: "mcw_to".to_string(),
        amount: 100.0,
        from_currency: "EUR".to_string(),
        to_currency: "GBP".to_string(),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("fromWallet"));
    assert!(json.contains("mcw_from"));
    assert!(json.contains("toWallet"));
    assert!(json.contains("mcw_to"));
    assert!(json.contains("amount"));
    assert!(json.contains("100.0"));
    assert!(json.contains("fromCurrency"));
    assert!(json.contains("EUR"));
    assert!(json.contains("toCurrency"));
    assert!(json.contains("GBP"));
}

#[test]
fn test_transfer_quote_response_deserialization() {
    let json = r#"{
        "status": "success",
        "exchangeRate": 0.85,
        "fee": 1.5,
        "sourceAmount": 100.0,
        "destinationAmount": 83.5
    }"#;
    let response: TransferQuoteResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.exchange_rate, Some(0.85));
    assert_eq!(response.fee, Some(1.5));
    assert_eq!(response.source_amount, Some(100.0));
    assert_eq!(response.destination_amount, Some(83.5));
}

#[test]
fn test_transfer_quote_response_optional_fields() {
    let json = r#"{
        "status": "success"
    }"#;
    let response: TransferQuoteResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert!(response.exchange_rate.is_none());
    assert!(response.fee.is_none());
    assert!(response.source_amount.is_none());
    assert!(response.destination_amount.is_none());
}

#[test]
fn test_multicurrency_transfer_request_serialization() {
    let request = MulticurrencyTransferRequest {
        from_wallet: "mcw_src".to_string(),
        recipient: "recv@test.com".to_string(),
        amount: 250.0,
        from_currency: "EUR".to_string(),
        to_currency: "USD".to_string(),
        note: Some("Monthly transfer".to_string()),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("fromWallet"));
    assert!(json.contains("mcw_src"));
    assert!(json.contains("recipient"));
    assert!(json.contains("recv@test.com"));
    assert!(json.contains("amount"));
    assert!(json.contains("250.0"));
    assert!(json.contains("fromCurrency"));
    assert!(json.contains("EUR"));
    assert!(json.contains("toCurrency"));
    assert!(json.contains("USD"));
    assert!(json.contains("note"));
    assert!(json.contains("Monthly transfer"));
}

#[test]
fn test_multicurrency_transfer_request_no_note() {
    let request = MulticurrencyTransferRequest {
        from_wallet: "mcw_nn".to_string(),
        recipient: "other@test.com".to_string(),
        amount: 50.0,
        from_currency: "GBP".to_string(),
        to_currency: "NGN".to_string(),
        note: None,
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("fromWallet"));
    assert!(json.contains("recipient"));
    assert!(json.contains("amount"));
    assert!(json.contains("fromCurrency"));
    assert!(json.contains("toCurrency"));
    assert!(!json.contains("note"));
}

#[test]
fn test_multicurrency_transfer_response_deserialization() {
    let json = r#"{
        "status": "success",
        "transactionId": "txn_mc_001",
        "message": "Transfer completed"
    }"#;
    let response: MulticurrencyTransferResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.transaction_id, Some("txn_mc_001".to_string()));
    assert_eq!(response.message, Some("Transfer completed".to_string()));
}

#[test]
fn test_multicurrency_transfer_response_minimal() {
    let json = r#"{
        "status": "pending"
    }"#;
    let response: MulticurrencyTransferResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.status, "pending");
    assert!(response.transaction_id.is_none());
    assert!(response.message.is_none());
}

#[test]
fn test_issue_wallet_request_serialization() {
    let request = IssueWalletRequest {
        sub_account: "sub_issue".to_string(),
        currency: "EUR".to_string(),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("subAccount"));
    assert!(json.contains("sub_issue"));
    assert!(json.contains("currency"));
    assert!(json.contains("EUR"));
}

#[test]
fn test_issue_bank_account_request_serialization() {
    let request = IssueBankAccountRequest {
        sub_account: "sub_bank".to_string(),
        country_code: "NG".to_string(),
        bank_code: "044".to_string(),
        account_number: "1234567890".to_string(),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("subAccount"));
    assert!(json.contains("sub_bank"));
    assert!(json.contains("countryCode"));
    assert!(json.contains("NG"));
    assert!(json.contains("bankCode"));
    assert!(json.contains("044"));
    assert!(json.contains("accountNumber"));
    assert!(json.contains("1234567890"));
}
