use std::collections::HashMap;
use crate::db::save_coins_to_db;
use crate::ui::{animate_text, clear_terminal, read_string, read_integer, show_coins};
use colored::*;
use std::io::Write;
use std::thread;
use std::time::Duration;

pub fn path() -> HashMap<String, i32> {
    let mut coin_map: HashMap<String, i32> = HashMap::new();
    clear_terminal();
    loop {
        animate_text("=============== CRYPTO CLI ===============".bold().to_string(), 25);
        animate_text("Menu: \n1 - Escolher Moeda  \n2 - Mostrar Suas Moedas  \n3 - Sair".bold().to_string(), 25);
        animate_text("=========================================".bold().to_string(), 25);
        animate_text("Escolha uma opção:".bold().to_string(), 25);
        print!("-> ");
        std::io::stdout().flush().unwrap(); 

        let input = read_string();
        if input == "1" {
            clear_terminal();
            animate_text("===== Adicione Moedas =====".bold().bright_yellow().to_string(), 25);
            let new_coins = choose_coin();
            coin_map.extend(new_coins);
        } else if input == "2" {
            clear_terminal();
            show_coins(&coin_map);
        } else if input == "3" {
            animate_text("\nSaindo... Obrigado por usar o Crypto CLI!".bold().bright_magenta().to_string(), 25);
            thread::sleep(Duration::from_secs(2));
            break;
        } else {
            animate_text("Opção inválida. Tente novamente.".bold().bright_red().to_string(), 25);
        }
    }
    coin_map
}

fn choose_coin() -> HashMap<String, i32> {
    let mut coin_map = HashMap::new();
    loop {
        animate_text("Qual moeda deseja adicionar?".bold().to_string(), 25);
        let coin = read_string();

        animate_text(
            format!("Deseja adicionar quantas {}?", coin.bold().bright_magenta()),
            25,
        );
        let quantity = read_integer();

        coin_map.insert(coin.clone(), quantity);

        if let Err(e) = save_coins_to_db(&coin_map) {
            animate_text(format!("Erro ao salvar moedas: {}", e).bold().bright_red().to_string(), 25);
        } else {
            animate_text(
                format!(
                    "{} {} {} com sucesso!",
                    "Adicionado".bold(),
                    quantity.to_string().bold().bright_yellow(),
                    coin.bold().bright_yellow()
                ),
                25,
            );
        }

        animate_text(
            "\nDeseja adicionar mais?\n1 - Sim\n2 - Não".bold().to_string(),
            25,
        );
        let input = read_string();
        if input == "2" {
            animate_text("Voltando ao menu principal...\n".bold().bright_magenta().to_string(), 25);
            thread::sleep(Duration::from_secs(2));
            break;
        }
    }
    coin_map
}

