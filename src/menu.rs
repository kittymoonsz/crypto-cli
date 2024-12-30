    use std::collections::HashMap;
use std::fs;
use crate::db::save_coins_to_db;
use crate::ui::{animate_text, clear_terminal, read_string, read_integer, show_coins};
use colored::*;
use std::io::Write;
use std::thread;
use std::time::Duration;
use crate::crypto::{get_url, Coin};
use rusqlite::{Connection, Result};
use tokio::time::{sleep};


pub async fn path() -> HashMap<String, f64> {
    let mut coin_map: HashMap<String, f64> = HashMap::new();
    clear_terminal();
    loop {
        animate_text("=============== CRYPTO CLI ===============".bold().to_string(), 25);
        animate_text("Menu: \n1 - Escolher Moeda  \n2 - Mostrar Suas Moedas  \n3 - Conversor\n4 - Sair".bold().to_string(), 25);
        animate_text("=========================================".bold().to_string(), 25);
        animate_text("Escolha uma opção:".bold().to_string(), 25);
        print!("-> ");
        std::io::stdout().flush().unwrap(); 

        let input = read_string();
        if input == "1" {
            clear_terminal();
            animate_text("===== Adicione Moedas =====".bold().bright_yellow().to_string(), 25);
            let new_coins = choose_coin().await;
            coin_map.extend(new_coins);
        } else if input == "2" {
            clear_terminal();
            show_coins(&coin_map);
        } else if input == "3" {
            clear_terminal();
            if let Err(e) = converter().await {
                animate_text(format!("Erro ao converter moedas: {}", e).bold().bright_red().to_string(), 25);
            }
        } else if input == "4" {
            animate_text("\nSaindo... Obrigado por usar o Crypto CLI!".bold().bright_magenta().to_string(), 25);
            thread::sleep(Duration::from_secs(2));
            break;
        } else {
            animate_text("Opção inválida. Tente novamente.".bold().bright_red().to_string(), 25);
        }
    }
    coin_map
}

async fn converter() -> Result<(), Box<dyn std::error::Error>> {
    // Abrir a conexão com o banco de dados
    let conn = Connection::open("coins.db")?;
    println!("Conectado ao banco de dados.");

    // Preparar a consulta SELECT para obter o nome e a quantidade das moedas
    let mut stmt = conn.prepare("SELECT name, quantity FROM coins")?;
    println!("Consulta preparada.");

    // Iterar sobre as moedas e obter nome e quantidade
    let moeda_iter = stmt.query_map([], |row| {
        let nome: String = row.get(0)?;
        let quantidade: f64 = row.get(1)?; // Alterado para f64 para manter precisão
        Ok((nome, quantidade)) // Retorna uma tupla com nome e quantidade
    })?;

    let mut total_usd = 0.0; // Variável para armazenar o valor total em USD

    let mut moeda_valores = Vec::new(); // Vetor para armazenar os valores totais de cada moeda

    // Iterar sobre cada moeda e calcular o valor total (preço * quantidade)
    for moeda in moeda_iter {
        let (nome, quantidade) = moeda?;

        // Obter o preço da moeda em USD
        match get_url(&nome).await {
            Ok(price) => {
                // Calcular o valor total da moeda (preço * quantidade)
                let valor_total = price * quantidade;

                // Adicionar o valor total da moeda ao valor global
                total_usd += valor_total;

                // Armazenar o valor total da moeda para exibição posterior
                moeda_valores.push((nome, quantidade, price, valor_total));
            }
            Err(err) => {
                println!("Erro ao obter o preço para {}: {}", nome, err.to_string());
            }
        }
    }

    // Exibir os valores individuais e as porcentagens com base no total correto
    for (nome, quantidade, price, valor_total) in moeda_valores {
        let porcentagem = (valor_total / total_usd) * 100.0;
        println!(
            "Moeda: {}, Quantidade: {}, Preço por unidade: ${}, Valor total: ${}, Porcentagem do total: {:.2}%",
            nome, quantidade, price, valor_total, porcentagem
        );
    }

    // Exibir o valor total em USD
    println!("Valor total em USD: ${}", total_usd);

    Ok(())
}

fn load_all_coins() -> Result<HashMap<String, Coin>, Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string("coins_list.json")?;
    let coins: HashMap<String, Coin> = serde_json::from_str(&file_content)?;
    Ok(coins)
}

async fn choose_coin() -> HashMap<String, f64> {
    let mut coin_map = HashMap::new();

    let all_coins = match load_all_coins() {
        Ok(coins) => coins,
        Err(_) => {
            animate_text("Erro ao carregar a lista de moedas.".to_string(), 25);
            return coin_map;
        }
    };

    loop {
        animate_text("Qual é o nome da moeda?".bold().to_string(), 25);
        let name_input = read_string();

        animate_text("Qual é o símbolo da moeda?".bold().to_string(), 25);
        let symbol_input = read_string();

        // Verificar correspondência pelo nome e símbolo
        if let Some((id, coin)) = all_coins.iter().find(|(_, coin)| {
            coin.name.to_lowercase() == name_input.to_lowercase()
                && coin.symbol.to_lowercase() == symbol_input.to_lowercase()
        }) {
            animate_text(
                format!(
                    "Você escolheu a moeda: {} ({})",
                    coin.name.bold(),
                    coin.symbol.bold()
                ),
                25,
            );
            animate_text(
                format!(
                    "Deseja adicionar quantas {}?",
                    coin.name.bold().bright_magenta()
                ),
                25,
            );

            let quantity = read_integer() as f64;

            coin_map.insert(id.clone(), quantity as f64);


            // Salvar no banco de dados
            if let Err(e) = save_coins_to_db(&coin_map) {
                animate_text(
                    format!("Erro ao salvar moedas: {}", e)
                    .bold()
                    .bright_red()
                    .to_string(),
                    25,
                );
            } else {
                animate_text(
                    format!(
                        "{} {} {} com sucesso!",
                        "Adicionado".bold(),
                        quantity.to_string().bold().bright_yellow(),
                        coin.name.bold().bright_yellow()
                    ),
                    25,
                );
            }

            animate_text(
                "\nDeseja adicionar mais?\n1 - Sim\n2 - Não"
                    .bold()
                    .to_string(),
                25,
            );
            let input = read_string();
            if input == "2" {
                animate_text(
                    "Voltando ao menu principal...\n"
                        .bold()
                        .bright_magenta()
                        .to_string(),
                    25,
                );
                break;
            }
        } else {
            animate_text(
                format!(
                    "{} não foi encontrado",
                    name_input.bold().bright_red()
                ),
                25,
            );
        }
    }

    coin_map
}

