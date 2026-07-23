use chimoney_rust_sdk::{ChimoneyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ChimoneyClient::new("your_api_key")?;

    let countries = client.get_airtime_countries().await?;
    println!("Airtime countries: {}", countries);

    let rates = client.get_exchange_rates().await?;
    println!("Exchange rates: {}", rates);

    let banks = client.get_banks("NG").await?;
    println!("Nigerian banks: {}", banks);

    Ok(())
}
