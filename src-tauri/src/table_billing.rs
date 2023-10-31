use rusqlite::{Connection, Result};

pub fn create_buyer(conn: &Connection, category_type: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO buyer (category_type, total, total_usd) VALUES (?1, 0.0, 0.0)",
        &[category_type],
    )?;
    Ok(())
}
pub fn check_buyer_records(conn: &Connection) -> Result<bool> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM buyer", [], |row| row.get(0))?;
    Ok(count > 0)
}
pub fn create_product_sell(
    conn: &Connection,
    id_product: i32,
    price: i32,
    usd_value: i32,
    amount: i32,
    buyer_id: i32,
) -> Result<()> {
    conn.execute(
        "INSERT INTO products_sell (id_product, price, usd_value, amount, total, total_usd, buyer) VALUES (?1, ?2, ?3, ?4, ?2 * ?4, ?2 * ?4 * ?3, ?5)",
        [id_product, price, usd_value, amount, buyer_id],
    )?;
    Ok(())
}
pub fn update_buyer_totals(conn: &Connection, buyer_id: i32) -> Result<()> {
    conn.execute(
        "UPDATE buyer
         SET total = (SELECT SUM(total) FROM products_sell WHERE buyer = ?1),
             total_usd = (SELECT SUM(total_usd) FROM products_sell WHERE buyer = ?1)
         WHERE id = ?1",
        [buyer_id],
    )?;
    Ok(())
}
