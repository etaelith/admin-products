use rusqlite::{params, Connection, OptionalExtension, Result};

use crate::{
    db_config::connect_database,
    item::{ItemForSell, PresellItems, ResponseDelete},
};

pub fn create_or_get_buyer_id(
    mut conn: Connection,
    category_type: &str,
    dni: Option<i64>,
    id_row: Option<i64>,
) -> Result<Option<i64>, rusqlite::Error> {
    let tx = conn.transaction()?;

    // Verificar si ya existe una fila con total igual a 0.0 y el category_type proporcionado
    let existing_id: Option<i64> = tx
        .query_row(
            "SELECT id FROM buyer WHERE id = ?1 ",
            params![id_row],
            |row| row.get(0),
        )
        .optional()?;

    if let Some(id) = existing_id {
        tx.commit()?;
        Ok(Some(id))
    } else {
        // Si no existe, crear una nueva fila con total igual a 0.0 y el category_type proporcionado
        tx.execute(
            "INSERT INTO buyer (category_type, dni, total, total_usd) VALUES (?1, ?2, 0.0, 0.0)",
            params![category_type, dni],
        )?;

        // Obtener el ID de la fila recién creada
        let last_insert_rowid = tx.last_insert_rowid();
        println!("{last_insert_rowid}");
        tx.commit()?;

        Ok(Some(last_insert_rowid))
    }
}

pub fn create_item_for_sell(
    conn: Connection,
    codebar: i64,
    amount: i64,
    usd: i64,
    id_row_table: i64,
) -> Result<ItemForSell, String> {
    let existing_id_item: i64 = match conn.query_row(
        "SELECT id FROM items WHERE codebar = ?1",
        [codebar as i32],
        |row| row.get(0),
    ) {
        Ok(id) => id,
        Err(_) => {
            println!("Error al obtener el ID del artículo");
            return Err(format!("Error al obtener el ID del artículo"));
        }
    };
    let current_stock: i64 = match conn.query_row(
        "SELECT stock FROM items WHERE codebar = ?1",
        [codebar as i32],
        |row| row.get(0),
    ) {
        Ok(stock) => stock,
        Err(_) => {
            println!("Error al obtener el stock del articulo");
            return Err(format!("Error al obtener el stock del artículo"));
        }
    };
    if current_stock >= amount {
        let price: i64 = match conn.query_row(
            "SELECT price FROM items WHERE codebar = ?1",
            [codebar],
            |row| row.get(0),
        ) {
            Ok(price) => price,
            Err(_) => {
                println!("Error al obtener el precio del articulo");
                return Err(format!("Error al obtener el precio del artículo"));
            }
        };

        let total = price * amount;
        let total_usd = total * usd;
        let existing_id: i64 = match conn.query_row(
            "SELECT id FROM buyer WHERE id = ?1",
            [id_row_table],
            |row| row.get(0),
        ) {
            Ok(id) => id,
            Err(_) => {
                println!("Error al obtener el ID del articulo");
                return Err(format!("Error al obtener el id "));
            }
        };
        if let Err(_) = conn.execute(
                "INSERT INTO presell (codebar, price, usd_value, amount, total, total_usd, buyer) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                [codebar, price, usd, amount, total, total_usd, existing_id],
            ) {
                println!("Error al  realizar la insercion del articulo");
                return Err(format!("Error al insertar en la base de datos"));
            }

        Ok(ItemForSell {
            id: existing_id_item,
            codebar,
            name: "NombreDelArticulo".to_string(), // Cambia esto a obtener el nombre real del artículo
            amount,
            price,
            usd_value: usd,
            total: total_usd,
        })
    } else {
        println!("No hay suficiente stock disponible");
        println!("No hay suficiente stock disponible");
        Err(format!(
            "No hay suficiente stock disponible (stock actual: {}, cantidad requerida: {})",
            current_stock, amount
        ))
    }
}
pub fn delete_item_sell(buyer_id: i64, codebar: i64) -> Result<ResponseDelete, rusqlite::Error> {
    let conn = connect_database();
    match conn {
        Ok(conn) => {
            match conn.execute(
                "DELETE FROM presell WHERE buyer = ?1 AND codebar = ?2",
                [buyer_id, codebar],
            ) {
                Ok(_) => Ok(ResponseDelete {
                    success: true,
                    error_message: None,
                }),
                Err(err) => Ok(ResponseDelete {
                    success: false,
                    error_message: Some(err.to_string()),
                }),
            }
        }
        Err(conn_err) => {
            // Error al conectar a la base de datos
            Err(conn_err)
        }
    }
}
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
pub fn commit_sell(buyer_id: i64) -> Result<(), rusqlite::Error> {
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
        }

        if let Err(update_error) = tx.execute(
            "UPDATE items SET stock = stock - ?1 WHERE codebar = ?2",
            params![row.amount, row.codebar],
        ) {
            eprintln!("Error en UPDATE: {:?}", update_error);
        }
    }

    if let Err(commit_error) = tx.commit() {
        eprintln!(
            "Error al hacer commit de la transacción: {:?}",
            commit_error
        );
        return Err(commit_error);
    }

    if let Err(update_error) = conn.execute(
        "UPDATE buyer SET total = ?1, total_usd = ?2 WHERE id = ?3",
        params![total_sum, total_sum_usd, buyer_id],
    ) {
        eprintln!("Error en UPDATE buyer: {:?}", update_error);
        return Err(update_error);
    }

    if let Err(delete_error) = conn.execute("DELETE FROM presell WHERE buyer = ?1", [buyer_id]) {
        eprintln!("Error en DELETE FROM presell: {:?}", delete_error);
        return Err(delete_error);
    }

    println!(
        "Terminé el for, \ntotal: {} \ntotal usd: {}",
        total_sum, total_sum_usd
    );
    Ok(())
}
