mod db;
mod crypto;
mod ui;
mod menu;

use rusqlite::Result;
use menu::path;
use db::save_coins_to_db;
use crypto::get_url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crypto_id = "bitcoin";
    let currency = "usd";

    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}",
        crypto_id, currency
    );

    if let Err(e) = get_url(&url, crypto_id, currency).await {
        eprintln!("Failed to fetch crypto price: {}", e);
    }

    let coin_map = path();

    if let Err(e) = save_coins_to_db(&coin_map) {
        eprintln!("Error saving coins: {}", e);
    }

    Ok(())
}

