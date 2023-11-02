// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db_config;
mod item;
mod table_billing;
mod table_item;
mod test;
use rusqlite::Result;

use crate::db_config::connect_database;
use db_config::setup_database;
use table_billing::SaleTransaction;
use table_item::{delete_item, get_items, save_item, update_prices};
use test::{create_item_for_sell, create_or_get_buyer_id};

/* Comandos TABLE items */
#[tauri::command]
fn save_to_database(codebar: &str, name: &str, stock: i64, price: f64, category: String) {
    let conn = setup_database().expect("Failed to open the database");
    save_item(&conn, codebar, &name, stock, price, &category)
        .expect("Failed to save to the database");
}
#[tauri::command]
fn get_items_db() -> Result<Vec<item::Item>, String> {
    get_items()
}
#[tauri::command]
fn update_prices_db(percent: u8) -> Result<(), String> {
    let conn = setup_database().map_err(|e| e.to_string())?;
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

/* Comandos TABLE products_sell & buyer */
#[tauri::command]
fn create_bill(category_type: &str) -> Result<(), String> {
    let conn = setup_database().expect("Failed to open the database");
    let mut sale_transaction = SaleTransaction::new(conn);

    sale_transaction
        .create_buyer(category_type)
        .map_err(|e| e.to_string())?;

    Ok(())
}
#[tauri::command]
fn add_item_table(codebar: i64, amount: i64, usd: i64) -> Result<(), String> {
    let conn = setup_database().map_err(|e| e.to_string())?;
    let mut sale_transaction = SaleTransaction::new(conn);

    // Añade el artículo a la venta utilizando el método de SaleTransaction
    sale_transaction
        .add_sale_item(codebar, amount, usd)
        .map_err(|e| e.to_string())?;

    Ok(())
}
#[tauri::command]
fn sell_completed() -> Result<(), String> {
    let conn = setup_database().map_err(|e| e.to_string())?;
    let mut sale_transaction = SaleTransaction::new(conn);
    sale_transaction.commit();
    Ok(())
}
/* comandos test */

#[tauri::command]
fn command_uno(uno: String, category_type: &str) -> Result<(), String> {
    println!("{uno}");
    let conn = connect_database().map_err(|e| e.to_string())?;
    create_or_get_buyer_id(conn, category_type);
    Ok(())
}

#[tauri::command]
fn command_dos(dos: String, codebar: i64, amount: i64, usd: i64) -> Result<(), String> {
    let conn = connect_database().map_err(|e| e.to_string())?;
    create_item_for_sell(conn, codebar, amount, usd);
    Ok(())
}

#[tauri::command]
fn command_tres(tres: String) {
    println!("{}", tres);
    let conn = connect_database().map_err(|e| e.to_string());
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_to_database,
            get_items_db,
            delete_item_db,
            update_prices_db,
            create_bill,
            add_item_table,
            sell_completed,
            command_uno,
            command_dos,
            command_tres
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
