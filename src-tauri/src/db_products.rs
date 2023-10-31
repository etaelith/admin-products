use super::item::Item;
use rusqlite::{Connection, Result};

pub fn setup_database() -> Result<Connection> {
    let conn = Connection::open("my_database.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            stock INTEGER,
            price INTEGER,
            description TEXT
        )",
        [],
    )?;
    conn.is_autocommit();
    Ok(conn)
}

pub fn save_item(
    conn: &Connection,
    name: &str,
    stock: u8,
    price: u16,
    description: &str,
) -> Result<()> {
    conn.execute(
        "INSERT INTO items (name, stock, price, description) VALUES (?1, ?2, ?3, ?4)",
        &[name, &stock.to_string(), &price.to_string(), description],
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
            let old_price: u64 = row.get(1)?;
            let new_price = (old_price as f64 * (1.0 + percent / 100.0)).round() as u16;
            Ok((id, new_price))
        })
        .map_err(|_e| rusqlite::Error::QueryReturnedNoRows)?;

    for result in mapped_rows {
        let (id, new_price) = result?;
        conn.execute(
            "UPDATE items SET price = ?1 WHERE id = ?2",
            &[&new_price, &(id as u16)],
        )?;
    }

    Ok(())
}

pub fn get_items() -> Result<Vec<Item>, String> {
    let conn = setup_database().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, stock, price, description FROM items")
        .map_err(|e| e.to_string())?;

    let mapped_rows = stmt
        .query_map([], |row| {
            Ok(Item {
                id: row.get(0)?,
                name: row.get(1)?,
                stock: row.get(2)?,
                price: row.get(3)?,
                description: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let items: Result<Vec<Item>, _> = mapped_rows.collect();

    items.map_err(|e| e.to_string())
}

pub fn delete_item(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
    conn.execute("DELETE FROM items WHERE id = ?1", &[&id])?;
    Ok(())
}
