use std::{thread, time, io::Write};
use colored::*;
use std::collections::HashMap;
use crate::db::get_coins_from_db;

pub fn animate_text(text: String, speed_ms: u64) {
    for c in text.chars() {
        print!("{}", c);
        thread::sleep(time::Duration::from_millis(speed_ms));
        std::io::stdout().flush().unwrap();
    }
    println!();
}

pub fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
   input.trim().to_string()
}

pub fn read_integer() -> i32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().parse().expect("Parsing error")
}

pub fn clear_terminal() {
    std::process::Command::new("clear")
        .status()
        .unwrap();
}

pub fn show_coins(coin_map: &HashMap<String, f64>) {
   animate_text("===== Suas Moedas =====".bold().bright_yellow().to_string(), 50);
    match get_coins_from_db() {
        Ok(coins) => {
            if coins.is_empty() {
                animate_text("Nenhuma moeda salva ainda.".bold().bright_red().to_string(), 50);
            } else {
                animate_text("================================".bright_blue().to_string(), 50);
                for (name, quantity) in coins {
                    animate_text(format!("{:<20} {:>10}", name.bright_cyan(), quantity), 50);
                }
                animate_text("================================".bright_blue().to_string(), 50);
            }
        },
        Err(e) => animate_text(format!("Erro ao carregar moedas: {}", e).bold().bright_red().to_string(), 50),
    }
}
