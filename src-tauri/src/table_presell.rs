use rusqlite::Connection;

use crate::{
    db_config::connect_database,
    item::{ItemForSell, ResponseStatus, ShowItemForSell},
};

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
        Err(format!(
            "No hay suficiente stock disponible (stock actual: {}, cantidad requerida: {})",
            current_stock, amount
        ))
    }
}
pub fn delete_item_sell(buyer_id: i64, codebar: i64) -> Result<ResponseStatus, rusqlite::Error> {
    let conn = connect_database();
    match conn {
        Ok(conn) => {
            match conn.execute(
                "DELETE FROM presell WHERE buyer = ?1 AND codebar = ?2",
                [buyer_id, codebar],
            ) {
                Ok(_) => Ok(ResponseStatus {
                    success: true,
                    error_message: None,
                }),
                Err(err) => Ok(ResponseStatus {
                    success: false,
                    error_message: Some(err.to_string()),
                }),
            }
        }
        Err(conn_err) => Err(conn_err),
    }
}
pub fn show_items_presell() -> Result<Vec<ShowItemForSell>, String> {
    let conn = connect_database().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, codebar, price, usd_value, amount, total FROM products_sell")
        .map_err(|e| e.to_string())?;

    let mapped_rows = stmt
        .query_map([], |row| {
            Ok(ShowItemForSell {
                id: row.get(0)?,
                codebar: row.get(1)?,
                price: row.get(2)?,
                usd_value: row.get(3)?,
                amount: row.get(4)?,
                total: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let items: Result<Vec<ShowItemForSell>, _> = mapped_rows.collect();

    items.map_err(|e| e.to_string())
}
