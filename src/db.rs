use rusqlite::{params, Connection, Result};
use std::collections::HashMap;

pub fn save_coins_to_db(coin_map: &HashMap<String, f64>) -> Result<()> {
    let connection = Connection::open("coins.db")?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS coins (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            quantity REAL NOT NULL
        )",
        [],
    )?;

    for (coin, quantity) in coin_map {
        connection.execute(
            "INSERT INTO coins (name, quantity) 
             VALUES (?1, ?2)
             ON CONFLICT(name) DO UPDATE SET 
             quantity = quantity + excluded.quantity",
            params![coin, quantity],
        )?;
    }

    Ok(())
}

pub fn get_coins_from_db() -> Result<HashMap<String, f64>> {
    let connection = Connection::open("coins.db")?;
    let mut stmt = connection.prepare("SELECT name, quantity FROM coins")?;

    let coin_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, f64>(1)?,
        ))
    });

    let mut coin_map = HashMap::new();
    match coin_iter {
        Ok(iter) => {
            for coin in iter {
                match coin {
                    Ok((coin_name, quantity)) => {
                        coin_map.insert(coin_name, quantity);
                    },
                    Err(e) => {
                        eprintln!("Erro ao ler a moeda: {}", e);
                    },
                }
            }
        },
        Err(e) => {
            eprintln!("Erro ao executar a consulta: {}", e);
        },
    }
    Ok(coin_map)
}
