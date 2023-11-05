// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db_config;
mod item;
mod table_billing;
mod table_item;
use item::{ItemForSell, ResponseDelete};
use rusqlite::Result;

use db_config::{connect_database, setup_database};
use table_billing::{
    cancel_sell, commit_sell, create_item_for_sell, create_or_get_buyer_id, delete_item_sell,
};
use table_item::{delete_item, get_items, save_item, update_prices};
use tauri::InvokeError;

/* Comandos TABLE items */

#[tauri::command]
fn save_to_database(codebar: &str, name: &str, stock: i64, price: f64, category: String) {
    let conn = connect_database().expect("Failed to open the database");
    save_item(&conn, codebar, &name, stock, price, &category)
        .expect("Failed to save to the database");
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
fn delete_item_db(id: String) {
    let num: i64 = id.parse().expect("Cant convert");
    let conn = setup_database().expect("Error when opened the db");
    delete_item(&conn, num).expect("Error on delete element");
    println!("Borraste este item {}", num)
}

/* comandos test */

#[tauri::command]
fn command_uno(
    category_type: &str,
    dni: Option<i64>,
    id_row: Option<i64>,
) -> Result<Option<i64>, String> {
    let conn = connect_database().map_err(|e| e.to_string())?;
    match create_or_get_buyer_id(conn, category_type, dni, id_row) {
        Ok(result) => Ok(result),
        Err(error) => Err(error.to_string()), // Convierte el error en una cadena de texto
    }
}

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
fn command_tres(buyer_id: i64) {
    commit_sell(buyer_id);
    println!("hola?");
}

#[tauri::command]
fn cancel_selldelete(buyer_id: i64) {
    cancel_sell(buyer_id);
}
#[tauri::command]
fn delete_item_specific(buyer_id: i64, codebar: i64) -> Result<ResponseDelete, InvokeError> {
    match delete_item_sell(buyer_id, codebar) {
        Ok(result) => Ok(result),
        Err(err) => {
            // Manejar el error y devolver un ResponseDelete con éxito en false y un mensaje de error
            Ok(ResponseDelete {
                success: false,
                error_message: Some(err.to_string()),
            })
        }
    }
}
fn main() {
    setup_database();
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
