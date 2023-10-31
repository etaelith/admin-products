use rusqlite::{Connection, Result};

use crate::{db_config::setup_database, item::Item};

pub fn save_item(
    conn: &Connection,
    codebar: &str,
    name: &str,
    stock: i32,
    price: f64,
    category: &str,
) -> Result<()> {
    conn.execute(
        "INSERT INTO items (codebar, name, stock, price, category, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
        &[codebar, name, &stock.to_string(), &price.to_string(), category],
    )?;
    Ok(())
}

pub fn update_prices(conn: &Connection, percent: f64) -> Result<(), rusqlite::Error> {
    let mut stmt = conn
        .prepare("SELECT id, price FROM items")
        .map_err(|_e| rusqlite::Error::QueryReturnedNoRows)?;

    let mapped_rows = stmt
        .query_map([], |row| {
            let id: i64 = row.get(0)?;
            let old_price: f64 = row.get(1)?;
            let new_price = (old_price * (1.0 + percent / 100.0)).round();
            Ok((id, new_price))
        })
        .map_err(|_e| rusqlite::Error::QueryReturnedNoRows)?;

    for result in mapped_rows {
        let (id, new_price) = result?;
        conn.execute(
            "UPDATE items SET price = ?1 WHERE id = ?2",
            [new_price, id as f64],
        )?;
    }

    Ok(())
}

pub fn get_items() -> Result<Vec<Item>, String> {
    let conn = setup_database().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, codebar, name, stock, price, category, created_at, updated_at FROM items",
        )
        .map_err(|e| e.to_string())?;

    let mapped_rows = stmt
        .query_map([], |row| {
            Ok(Item {
                id: row.get(0)?,
                codebar: row.get(1)?,
                name: row.get(2)?,
                stock: row.get(3)?,
                price: row.get(4)?,
                category: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let items: Result<Vec<Item>, _> = mapped_rows.collect();

    items.map_err(|e| e.to_string())
}

pub fn delete_item(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
    conn.execute("DELETE FROM items WHERE id = ?1", [id])?;
    Ok(())
}
