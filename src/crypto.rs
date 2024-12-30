use reqwest;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
use crate::crypto::reqwest::StatusCode;


#[derive(Debug, Deserialize, Serialize)]
pub struct Coin {
    pub id: String,
    pub name: String,
    pub symbol: String,
}

pub async fn get_url(nome: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd", nome);
    
    // Número máximo de tentativas
    let max_retries = 50;
    // Intervalo inicial para espera
    let wait_time = Duration::from_secs(5);

    for attempt in 0..max_retries {
        let response = reqwest::get(&url).await;
        match response {
            Ok(res) => {
                if res.status().is_success() {
                    let price: serde_json::Value = res.json().await?;
                    if let Some(price_value) = price.get(nome) {
                        if let Some(usd_value) = price_value.get("usd") {
                            // Mensagem indicando que a requisição para essa moeda foi bem-sucedida
                            println!("Preço da moeda '{}' obtido com sucesso: ${:.2}", nome, usd_value.as_f64().unwrap_or(0.0));
                            return Ok(usd_value.as_f64().unwrap_or(0.0));
                        }
                    }
                } else {
                    // Caso a resposta não seja bem-sucedida, apenas verifica o código de status, mas sem imprimir erros
                    if res.status() == StatusCode::TOO_MANY_REQUESTS {
                        // Se o limite for atingido, apenas aguarda e tenta novamente
                        sleep(wait_time).await;
                    }
                }
            }
            Err(_) => {
                // Não exibe erro para falhas na requisição, mas continuará tentando
            }
        }

        // Aumenta o tempo de espera de forma exponencial
        sleep(wait_time).await;

        // Se atingiu o máximo de tentativas, retorna erro sem imprimir mensagem de erro
        if attempt == max_retries - 1 {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Erro ao obter o preço após várias tentativas")));
        }
    }
    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Erro ao obter o preço")))
}

pub async fn get_all_coins() -> Result<Vec<Coin>, Error> {
    let url = "https://api.coingecko.com/api/v3/coins/list";
    let response = reqwest::get(url).await?;
    let coins: Vec<Coin> = response.json().await?;
    Ok(coins)
}

