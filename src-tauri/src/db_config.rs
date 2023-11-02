use rusqlite::{Connection, Result};

pub fn setup_database() -> Result<Connection> {
    let conn = Connection::open("my_database.db")?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY,
            codebar INTEGER NOT NULL,
            name TEXT NOT NULL,
            stock INTEGER,
            price INTEGER,  -- Usar REAL para valores decimales
            category TEXT,  -- Campo de categoría
            created_at TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE IF NOT EXISTS products_sell (
            id INTEGER PRIMARY KEY,
            codebar TEXT NOT NULL,
            price INTEGER,
            usd_value INTEGER,
            amount INTEGER,
            total INTEGER,
            total_usd INTEGER,
            buyer INTEGER,
            FOREIGN KEY (codebar) REFERENCES items (codebar),
            FOREIGN KEY (buyer) REFERENCES buyer (id)
        );
        CREATE TABLE IF NOT EXISTS buyer (
            id INTEGER PRIMARY KEY,
            category_type TEXT, -- Cambiado a texto para una letra
            total INTEGER,
            total_usd INTEGER
        );
        CREATE TABLE IF NOT EXISTS presell (
            id INTEGER PRIMARY KEY,
            codebar INTEGER NOT NULL,
            amount INTEGER NOT NULL,
            price INTEGER NOT NULL,
            usd_value INTEGER NOT NULL,
            total INTEGER NOT NULL,
            total_usd INTEGER NOT NULL,
            buyer INTEGER NOT NULL,
            FOREIGN KEY (codebar) REFERENCES items(codebar),
            FOREIGN KEY (buyer) REFERENCES buyer (id)
        );",
    )?;
    conn.is_autocommit();
    Ok(conn)
}

pub fn connect_database() -> Result<Connection> {
    let conn = Connection::open("my_database.db")?;
    Ok(conn)
}
