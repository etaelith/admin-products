use rusqlite::{Connection, Result};

pub fn setup_database() -> Result<Connection> {
    let conn = Connection::open("my_database.db")?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY,
            codebar TEXT NOT NULL,
            name TEXT NOT NULL,
            stock INTEGER,
            price REAL,  -- Usar REAL para valores decimales
            category TEXT,  -- Campo de categoría
            created_at TIMESTAMP,
            updated_at DATETIME  -- Corregir el tipo de datos
        );
        CREATE TABLE IF NOT EXISTS products_sell (
            id INTEGER PRIMARY KEY,
            id_product INTEGER,
            price REAL,
            usd_value REAL,
            amount INTEGER,
            total REAL,
            total_usd REAL,
            buyer INTEGER,
            FOREIGN KEY (id_product) REFERENCES items (id),
            FOREIGN KEY (buyer) REFERENCES buyer (id)
        );
        CREATE TABLE IF NOT EXISTS buyer (
            id INTEGER PRIMARY KEY,
            category_type TEXT, -- Cambiado a texto para una letra
            total REAL,
            total_usd REAL
        );",
    )?;
    conn.is_autocommit();
    Ok(conn)
}
