use reqwest;
use serde_json::Value;

pub async fn get_url(url: &str, crypto_id: &str, currency: &str) -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;
    let json: Value = serde_json::from_str(&body)?;

    if let Some(price) = json.get(crypto_id).and_then(|v| v.get(currency)) {
        println!("Price of {}: {} {}", crypto_id, price, currency.to_uppercase());
    } else {
        println!("Could not find the price for {} in {}", crypto_id, currency);
    }
    Ok(())
}

