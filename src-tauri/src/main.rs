// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db_config;
mod item;
mod table_buyer;
mod table_commit;
mod table_item;
mod table_presell;

use db_config::{connect_database, setup_database};
use item::{ItemForSell, ResponseStatus};
use rusqlite::Result;
use table_buyer::{create_buyer_id, get_buyer_id};
use table_commit::{cancel_sell, commit_sell};
use table_item::{delete_item, get_items, save_item, update_prices};
use table_presell::{create_item_for_sell, delete_item_sell};
use tauri::InvokeError;

/* Comandos TABLE items */

#[tauri::command]
fn save_to_database(
    codebar: String,
    name: String,
    stock: i64,
    price: f64,
    category: String,
) -> ResponseStatus {
    let conn = connect_database().expect("Failed to open the database");
    match save_item(&conn, &codebar, &name, stock, price, &category) {
        response @ ResponseStatus { success: true, .. } => response,
        ResponseStatus {
            success: false,
            error_message: Some(error),
        } => ResponseStatus {
            success: false,
            error_message: Some(error),
        },
        _ => ResponseStatus {
            success: false,
            error_message: Some("Unknown error".to_string()),
        },
    }
}

#[tauri::command]
fn get_items_db() -> Result<Vec<item::Item>, String> {
    get_items()
}
#[tauri::command]
fn update_prices_db(percent: u8) -> Result<(), String> {
    let conn = connect_database().map_err(|e| e.to_string())?;
    update_prices(&conn, percent as f64).map_err(|e| e.to_string())?;
    Ok(())
}
#[tauri::command]
fn delete_item_db(id: i64) {
    let conn = connect_database().expect("Error when opened the db");
    delete_item(&conn, id).expect("Error on delete element");
    println!("Borraste este item {}", id)
}

/* COMANDOS VENTA */
/* Buscar Boleta */
#[tauri::command]
fn search_boleta(id_row: i64) -> Result<ResponseStatus, InvokeError> {
    match get_buyer_id(id_row) {
        Ok(response) => Ok(response),
        Err(error) => Ok(ResponseStatus {
            success: false,
            error_message: Some(error.to_string()),
        }),
    }
}
/* Crear Boleta */
#[tauri::command]
fn command_uno(category_type: &str, dni: i64) -> Result<i64, String> {
    let conn = connect_database().map_err(|e| e.to_string())?;
    match create_buyer_id(conn, category_type, dni) {
        Ok(result) => Ok(result),
        Err(error) => Err(error.to_string()),
    }
}
/* Agregar Producto a la boleta */
#[tauri::command]
fn command_dos(
    codebar: i64,
    amount: i64,
    usd: i64,
    id_row_table: i64,
) -> Result<ItemForSell, String> {
    let conn = connect_database().map_err(|e| e.to_string())?;
    let result = create_item_for_sell(conn, codebar, amount, usd, id_row_table);

    result.map_err(|e| e.to_string())
}
#[tauri::command]
fn cancel_selldelete(buyer_id: i64) {
    let _ = cancel_sell(buyer_id);
}
#[tauri::command]
fn command_tres(buyer_id: i64) -> Result<ResponseStatus, InvokeError> {
    match commit_sell(buyer_id) {
        Ok(_) => {
            println!("hola?");
            Ok(ResponseStatus {
                success: true,
                error_message: None,
            })
        }
        Err(error) => {
            eprintln!("Error en commit_sell: {:?}", error);
            Ok(ResponseStatus {
                success: false,
                error_message: Some(format!("Error en commit_sell: {:?}", error)),
            })
        }
    }
}

#[tauri::command]
fn delete_item_specific(buyer_id: i64, codebar: i64) -> Result<ResponseStatus, InvokeError> {
    match delete_item_sell(buyer_id, codebar) {
        Ok(result) => Ok(result),
        Err(err) => {
            // Manejar el error y devolver un ResponseStatus con éxito en false y un mensaje de error
            Ok(ResponseStatus {
                success: false,
                error_message: Some(err.to_string()),
            })
        }
    }
}
fn main() {
    let _ = setup_database();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_to_database,
            get_items_db,
            delete_item_db,
            update_prices_db,
            command_uno,
            command_dos,
            command_tres,
            cancel_selldelete,
            delete_item_specific,
            search_boleta
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
