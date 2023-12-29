use rusqlite::{params, Connection, Result};

use crate::{
    db_config::connect_database,
    item::{BuyerRecord, ResponseStatus},
};

pub fn get_buyer_id(id_row: i64) -> Result<ResponseStatus, rusqlite::Error> {
    let conn = connect_database().expect("Failed to open the database");

    let result = conn.query_row("SELECT id FROM buyer WHERE id = ?1", [id_row], |row| {
        let _buyer_id: i64 = row.get(0)?;
        // You can retrieve other fields from the row in a similar manner

        Ok(ResponseStatus {
            success: true,
            error_message: None,
        })
    });

    match result {
        Ok(response) => Ok(response),
        Err(error) => {
            if error == rusqlite::Error::QueryReturnedNoRows {
                // Handle the case where no rows were returned
                Ok(ResponseStatus {
                    success: false,
                    error_message: Some("No se encontro la boleta".to_string()),
                })
            } else {
                // Handle other database errors and return an appropriate ResponseStatus
                Ok(ResponseStatus {
                    success: false,
                    error_message: Some(format!("Database error: {}", error)),
                })
            }
        }
    }
}

pub fn create_buyer_id(
    mut conn: Connection,
    category_type: &str,
    dni: i64,
) -> Result<i64, rusqlite::Error> {
    let tx = conn.transaction()?;
    tx.execute(
        "INSERT INTO buyer (category_type, dni, total, total_usd) VALUES (?1, ?2, 0.0, 0.0)",
        params![category_type, dni],
    )?;

    // Obtener el ID de la fila recién creada
    let last_insert_rowid = tx.last_insert_rowid();

    tx.commit()?;

    Ok(last_insert_rowid)
}

pub fn show_buyers() -> Result<Vec<BuyerRecord>, String> {
    let conn = connect_database().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, category_type, dni, total, total_usd FROM buyer")
        .map_err(|e| e.to_string())?;

    let mapped_rows = stmt
        .query_map([], |row| {
            Ok(BuyerRecord {
                id: row.get(0)?,
                category_type: row.get(1)?,
                dni: row.get(2)?,
                total: row.get(3)?,
                total_usd: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let buyers: Result<Vec<BuyerRecord>, _> = mapped_rows.collect();

    buyers.map_err(|e| e.to_string())
}
