use rusqlite::{params, Connection, OptionalExtension, Result};

pub fn create_or_get_buyer_id(mut conn: Connection, category_type: &str) -> Result<i64> {
    let tx = conn.transaction()?;

    // Verificar si ya existe una fila con total igual a 0.0 y el category_type proporcionado
    let existing_id: Option<i64> = tx
        .query_row(
            "SELECT id FROM buyer WHERE total = 0.0 AND category_type = ?1",
            params![category_type],
            |row| row.get(0),
        )
        .optional()?;

    if let Some(id) = existing_id {
        tx.commit()?;
        Ok(id)
    } else {
        // Si no existe, crear una nueva fila con total igual a 0.0 y el category_type proporcionado
        tx.execute(
            "INSERT INTO buyer (category_type, total, total_usd) VALUES (?1, 0.0, 0.0)",
            params![category_type],
        )?;

        // Obtener el ID de la fila recién creada
        let last_insert_rowid = tx.last_insert_rowid();
        tx.commit()?;

        Ok(last_insert_rowid)
    }
}

pub fn create_item_for_sell(
    mut conn: Connection,
    codebar: i64,
    amount: i64,
    usd: i64,
) -> Result<()> {
    let current_stock: i64 = conn.query_row(
        "SELECT stock FROM items WHERE codebar = ?1",
        [codebar as i32],
        |row| row.get(0),
    )?;
    if current_stock >= amount {
        let price: i64 = conn.query_row(
            "SELECT price FROM items WHERE codebar = ?1",
            [codebar],
            |row| row.get(0),
        )?;

        let total = price * amount;
        let total_usd = total * usd;
        let existing_id: i64 = conn
            .query_row("SELECT id FROM buyer WHERE total = 0", [], |row| row.get(0))
            .unwrap();
        println!("{:?}", existing_id);
        conn.execute(
            "INSERT INTO presell (codebar, price, usd_value, amount, total, total_usd, buyer) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            [codebar,price,usd,amount,total,total_usd,existing_id]);
        Ok(())
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}
pub fn commit_sell(mut conn: Connection) {
    let mut stmt = conn
        .prepare("SELECT * FROM preseller WHERE buyer = ?1")
        .map_err(|e| rusqlite::Error::QueryReturnedNoRows);
}
