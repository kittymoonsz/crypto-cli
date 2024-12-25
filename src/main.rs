#![allow(unused)]

use serde_json::Value;
use reqwest;
use tokio;
use std::process::Command;
use std::collections::HashMap;
use rusqlite::{params, Connection, Result};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crypto_id = "bitcoin";
    let currency = "usd";
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}",
        crypto_id, currency
    );
    //get_url(&url, crypto_id, currency).await?;
    let coin_map = path();

    if let Err(e) = save_coins_to_db(&coin_map) {
        eprintln!("Error saving coins: {}", e);
    }
    Ok(())
}

fn path() -> HashMap<String, i32> {
    let mut coin_map: HashMap<String, i32> = HashMap::new();
    loop {
        println!("Welcome!\n1 - Escolher Moeda\n2 - Mostrar Suas Moedas\n3 - Sair");
        let input = read_string();
        if input == "1" {
            clear_terminal();
            println!("===== Adicione Moedas =====");
            coin_map = choose_coin();  // Now it properly updates the coin_map
        }
        else if input == "2" {
            clear_terminal();
            show_coins(); 
        }
        else if input == "3" {
            break;
        }
    }
    coin_map  // Return the updated coin_map correctly
}
fn choose_coin() -> HashMap<String, i32> {
    let mut coin_map: HashMap<String, i32> = HashMap::new();
    loop {
        println!("Qual moeda deseja adicionar?");
        let coin = read_string();

        println!("Quantas {} você tem?", coin);
        let quantity = read_integer();

        coin_map.insert(coin, quantity);

        println!("\nDeseja adicionar mais?\n1 - Sim\n2 - Não\n");
        let input = read_string();
        if input == "1" {
            continue; 
        }
        else if input == "2" {
            println!("{:#?}", coin_map);
            return coin_map 
        }
    }
}


fn show_coins() {
    match get_coins_from_db() {
        Ok(coins) => {
            for (name, quantity) in coins {
                println!("{}: {}", name, quantity);
            }
        },
        Err(e) => println!("Erro ao carregar moedas: {}", e),
    }
}

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
   input.trim().to_string()
}

fn read_integer() -> i32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().parse().expect("Parsing error")
}

fn clear_terminal() {
    Command::new("clear")
        .status()
        .unwrap();
}

async fn get_url(url: &str, crypto_id: &str, currency: &str) -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;
    println!("url: {:#?}", url);
    let json: Value = serde_json::from_str(&body)?;
    println!("json = {:#?}", json);

    if let Some(price) = json.get(crypto_id).and_then(|v| v.get(currency)) {
        println!("Price of {}: {} {}", crypto_id, price, currency.to_uppercase());
    } else {
        println!("Could not find the price for {} in {}", crypto_id, currency);

    }
    Ok(())
}

fn save_coins_to_db(coin_map: &HashMap<String, i32>) -> Result<()> {
    let connection = Connection::open("coins.db")?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS coins (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            quantity INTEGER NOT NULL
            )",
            [],
        )?;

    for(coin, quantity) in coin_map {
        connection.execute(
            "INSERT INTO coins (name, quantity) VALUES (?1, ?2)",
            params![coin, quantity],
            )?;
    }
    println!("Moedas salvas com sucesso!");
    Ok(())
}

fn get_coins_from_db() -> Result<HashMap<String, i32>> {
    let connection = Connection::open("coins.db")?;
    let mut stmt = connection.prepare("SELECT name, quantity FROM coins")?;
    
    // Aqui, o `query_map` é tratado manualmente.
    let coin_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, i32>(1)?,
        ))
    });

    // Vamos usar `match` para tratar o erro de forma explícita.
    let mut coin_map = HashMap::new();
    match coin_iter {
        Ok(iter) => {
            for coin in iter {
                match coin {
                    Ok((coin_name, quantity)) => {
                        coin_map.insert(coin_name, quantity);
                    },
                    Err(e) => {
                        println!("Erro ao ler a moeda: {}", e);
                    },
                }
            }
        },
        Err(e) => {
            println!("Erro ao executar a consulta: {}", e);
        },
    }

    Ok(coin_map)
}

