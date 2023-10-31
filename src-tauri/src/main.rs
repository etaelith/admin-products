// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db_config;
mod item;
mod table_billing;
mod table_item;

use db_config::setup_database;
use rusqlite::Result;
use table_billing::{check_buyer_records, create_buyer, create_product_sell};
use table_item::{delete_item, get_items, save_item, update_prices};

/* Comandos TABLE items */
#[tauri::command]
fn save_to_database(codebar: &str, name: &str, stock: i32, price: f64, category: String) {
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
fn create_bill(category_type: &str) {
    println!("{category_type}");
    let conn = setup_database().expect("Failed to open the database");
    create_buyer(&conn, category_type).expect("Error creando boleta")
}
#[tauri::command]
fn check_buyer_records_db() -> Result<bool, String> {
    let conn = setup_database().map_err(|e| e.to_string())?;
    check_buyer_records(&conn).map_err(|e| e.to_string())
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_to_database,
            get_items_db,
            delete_item_db,
            update_prices_db,
            create_bill,
            check_buyer_records_db
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
