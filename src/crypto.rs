use reqwest;
use serde_json::Value;
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Coin {
    pub id: String,
    pub name: String,
    pub symbol: String,
}

pub async fn get_url(crypto_id: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd", crypto_id);
    let body = reqwest::get(&url)
        .await?
        .text()
        .await?;

    let json: Value = serde_json::from_str(&body)?;

    if let Some(price) = json.get(crypto_id).and_then(|coin| coin.get("usd")) {
        let price_value = price.as_f64().unwrap_or(0.0);
        return Ok(price_value);
    } else {
        Err("Não foi possível encontrar o preço.".into())
    }
}


pub async fn get_all_coins() -> Result<Vec<Coin>, Error> {
    let url = "https://api.coingecko.com/api/v3/coins/list";
    let response = reqwest::get(url).await?;
    let coins: Vec<Coin> = response.json().await?;
    Ok(coins)
}

