mod db;
mod crypto;
mod ui;
mod menu;

use rusqlite::Result;
use crate::crypto::Coin;
use std::collections::HashMap;
use crate::menu::path;
use crypto::get_all_coins;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    path().await;
    coin_list().await?;
    Ok(())
}

async fn coin_list() -> Result<(), Box<dyn std::error::Error>> {
    let all_coins = match get_all_coins().await {
        Ok(coins) => coins,
        Err(e) => {
            eprintln!("Failed to fetch all coins: {}", e);
            return Ok(());
        }
    };

    println!("Fetched {} coins.", all_coins.len());

    let coin_map: HashMap<String, Coin> = all_coins
        .into_iter()
        .map(|coin| (coin.id.clone(), coin))
        .collect();

    let file_content = serde_json::to_string_pretty(&coin_map)?;
    let mut file = File::create("coins_list.json")?;
    file.write_all(file_content.as_bytes())?;

    println!("Saved list of all coins to 'coins_list.json'.");
    Ok(())
}

