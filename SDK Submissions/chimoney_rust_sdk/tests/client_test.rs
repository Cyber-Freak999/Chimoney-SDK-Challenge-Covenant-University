use chimoney_rust_sdk::{ChimoneyClient, ChimoneyError};

#[test]
fn test_client_new_with_valid_key() {
    let client = ChimoneyClient::new("test_api_key_12345");
    assert!(client.is_ok());
    let client = client.unwrap();
    assert_eq!(client.base_url(), "https://api.chimoney.io");
}

#[test]
fn test_client_new_with_empty_key() {
    let result = ChimoneyClient::new("");
    assert!(result.is_err());
    match result {
        Err(ChimoneyError::ApiKeyEmpty) => {}
        Err(other) => panic!("Expected ApiKeyEmpty, got {:?}", other),
        Ok(_) => panic!("Expected error, got Ok"),
    }
}

#[test]
fn test_client_new_sandbox() {
    let client = ChimoneyClient::new_sandbox("test_api_key_12345").unwrap();
    assert_eq!(client.base_url(), "https://api-v2-sandbox.chimoney.io");
}

#[test]
fn test_client_builder_default_values() {
    let client = ChimoneyClient::builder("test_key").build().unwrap();
    assert_eq!(client.base_url(), "https://api.chimoney.io");
}

#[test]
fn test_client_builder_custom_base_url() {
    let client = ChimoneyClient::builder("test_key")
        .base_url("https://custom.api.example.com")
        .build()
        .unwrap();
    assert_eq!(client.base_url(), "https://custom.api.example.com");
}

#[test]
fn test_client_builder_max_retries() {
    let client = ChimoneyClient::builder("test_key")
        .max_retries(5)
        .build();
    assert!(client.is_ok());
}

#[test]
fn test_client_builder_timeout() {
    let client = ChimoneyClient::builder("test_key")
        .timeout(60)
        .build();
    assert!(client.is_ok());
}

#[test]
fn test_client_http_client_accessible() {
    let client = ChimoneyClient::new("test_key").unwrap();
    let _http = client.http_client();
}

#[test]
fn test_client_chained_builder() {
    let client = ChimoneyClient::builder("test_key")
        .base_url("https://example.com")
        .max_retries(3)
        .timeout(30)
        .build()
        .unwrap();
    assert_eq!(client.base_url(), "https://example.com");
}
