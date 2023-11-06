use rusqlite::{params, Connection, Result};

use crate::{
    db_config::connect_database,
    item::{PresellItems, ResponseStatus},
};

pub fn get_items_presell(
    conn: &Connection,
    buyer_id: i64,
) -> Result<Vec<PresellItems>, rusqlite::Error> {
    let mut stmt = conn
        .prepare("SELECT codebar, price, usd_value, amount, total, total_usd, buyer FROM presell WHERE buyer = ?1")?;

    let mapped_rows = stmt.query_map([buyer_id], |row| {
        Ok(PresellItems {
            codebar: row.get(0)?,
            price: row.get(1)?,
            usd_value: row.get(2)?,
            amount: row.get(3)?,
            total: row.get(4)?,
            total_usd: row.get(5)?,
            buyer: row.get(6)?,
        })
    })?;

    let items: Result<Vec<PresellItems>, rusqlite::Error> = mapped_rows.collect();
    items
}
pub fn cancel_sell(buyer_id: i64) -> Result<(), rusqlite::Error> {
    let conn = connect_database()?;
    conn.execute("DELETE FROM presell WHERE buyer = ?1", [buyer_id])?;
    conn.execute("DELETE FROM buyer WHERE id = ?1 AND total = 0", [buyer_id])?;
    Ok(())
}
pub fn commit_sell(buyer_id: i64) -> Result<ResponseStatus, rusqlite::Error> {
    let mut conn = connect_database()?;
    println!("Me conecté");
    let datos = get_items_presell(&conn, buyer_id)?;
    println!("Pasé datos");
    println!("Datos: {:?}", datos);
    let mut total_sum: i64 = 0;
    let mut total_sum_usd: i64 = 0;
    let tx = conn.transaction()?;

    for row in datos {
        let row = row;
        total_sum += row.total;
        total_sum_usd += row.total_usd;
        println!("Estoy en el ciclo for");
        println!("{:?}", row);

        if let Err(insert_error) = tx.execute(
            "INSERT INTO products_sell (codebar, price, usd_value, amount, total, total_usd, buyer) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![row.codebar, row.price, row.usd_value, row.amount, row.total, row.total_usd, row.buyer],
        ) {
            eprintln!("Error en INSERT: {:?}", insert_error);
            return Err(insert_error); // Devuelve el error y sale de la función
        }

        if let Err(update_error) = tx.execute(
            "UPDATE items SET stock = stock - ?1 WHERE codebar = ?2",
            params![row.amount, row.codebar],
        ) {
            eprintln!("Error en UPDATE: {:?}", update_error);
            return Err(update_error); // Devuelve el error y sale de la función
        }
    }

    if let Err(commit_error) = tx.commit() {
        eprintln!(
            "Error al hacer commit de la transacción: {:?}",
            commit_error
        );
        return Err(commit_error); // Devuelve el error y sale de la función
    }

    if let Err(update_error) = conn.execute(
        "UPDATE buyer SET total = ?1, total_usd = ?2 WHERE id = ?3",
        params![total_sum, total_sum_usd, buyer_id],
    ) {
        eprintln!("Error en UPDATE buyer: {:?}", update_error);
        return Err(update_error); // Devuelve el error y sale de la función
    }

    if let Err(delete_error) = conn.execute("DELETE FROM presell WHERE buyer = ?1", [buyer_id]) {
        eprintln!("Error en DELETE FROM presell: {:?}", delete_error);
        return Err(delete_error); // Devuelve el error y sale de la función
    }

    println!(
        "Terminé el for, \ntotal: {} \ntotal usd: {}",
        total_sum, total_sum_usd
    );

    // Si llegamos aquí, todo ha tenido éxito
    Ok(ResponseStatus {
        success: true,
        error_message: None,
    })
}
