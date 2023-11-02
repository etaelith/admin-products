use crate::item::SaleItem;
use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct SaleTransaction {
    pub conn: Connection,
    pub sale_items: Vec<SaleItem>,
    pub buyer_id: Option<i64>,
}
static STATIC_BUYER_ID: Option<i64> = Some(1);
/* Transaction */

impl SaleTransaction {
    pub fn new(conn: Connection) -> SaleTransaction {
        SaleTransaction {
            conn,
            sale_items: vec![],
            buyer_id: None,
        }
    }

    pub fn create_buyer(&mut self, category_type: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO buyer (category_type, total, total_usd) VALUES (?1, 0.0, 0.0)",
            &[category_type],
        )?;
        self.buyer_id = Some(self.conn.last_insert_rowid());
        Ok(())
    }

    pub fn add_sale_item(&mut self, codebar: i64, amount: i64, usd: i64) -> Result<()> {
        // Verificar si hay suficiente stock para la venta

        let current_stock: i64 = self.conn.query_row(
            "SELECT stock FROM items WHERE codebar = ?1",
            [codebar as i32],
            |row| row.get(0),
        )?;
        println!("CurrentStock {current_stock}, amount: {amount}");
        if current_stock >= amount {
            println!(
                "Current mayor que amount, current_stock: {}, amount: {}",
                current_stock, amount
            );
            let price: i64 = self.conn.query_row(
                "SELECT price FROM items WHERE codebar = ?1",
                [codebar],
                |row| row.get(0),
            )?;

            if let Some(buyer_id) = self.buyer_id.or(STATIC_BUYER_ID) {
                let sale_item = SaleItem {
                    codebar,
                    amount,
                    price,
                    usd_value: usd,
                    buyer: buyer_id,
                };
                println!("{:?}", sale_item);
                self.sale_items.push(sale_item);

                Ok(())
            } else {
                println!("Esta aca el error");
                return Err(rusqlite::Error::QueryReturnedNoRows);
            }
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }

    pub fn commit(&mut self) -> Result<()> {
        let transaction = self.conn.transaction()?;
        println!("{:?}", self.sale_items);
        for item in &self.sale_items {
            if let Some(buyer_id) = self.buyer_id {
                // Actualizar el stock en la tabla items
                let current_stock: i64 = transaction.query_row(
                    "SELECT stock FROM items WHERE codebar = ?1",
                    [item.codebar],
                    |row| row.get(0),
                )?;
                let new_stock = current_stock - item.amount;
                println!("{new_stock}, codebar: {}", item.codebar);
                transaction.execute(
                    "UPDATE items SET stock = ?1 WHERE codebar = ?2",
                    [new_stock, item.codebar],
                )?;

                // Insertar la venta en la tabla products_sell con el ID del comprador
                transaction.execute(
                    "INSERT INTO products_sell (codebar, price, usd_value, amount, total, total_usd, buyer) VALUES (?1, ?2, ?3, ?4, ?5 * ?2 * ?3 , ?5 * ?2 , ?6)",
                    [item.codebar,item.amount,item.price,item.usd_value,item.amount,buyer_id as i64],
                )?;
            } else {
                println!("transaction rollback last");
                transaction.rollback()?;
                return Err(rusqlite::Error::QueryReturnedNoRows);
            }
        }

        // Confirmar la transacción
        transaction.commit()?;
        self.sale_items.clear();
        self.buyer_id = None;
        println!("termine el commit");
        Ok(())
    }
}

/* pub fn create_product_sell(
    conn: &Connection,
    id_product: i32,
    price: f64,
    usd_value: f64,
    amount: i32,
    buyer_id: i32,
) -> Result<()> {
    conn.execute(
        "INSERT INTO products_sell (id_product, price, usd_value, amount, total, total_usd, buyer) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        [id_product, price, usd_value, amount,price * amount as f64,price * usd_value * amount as f64,  buyer_id],
    )?;
    Ok(())
} */
/* pub fn create_buyer(conn: &Connection, category_type: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO buyer (category_type, total, total_usd) VALUES (?1, 0.0, 0.0)",
        &[category_type],
    )?;
    Ok(())
}
pub fn check_buyer_records(conn: &Connection) -> Result<bool> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM buyer", [], |row| row.get(0))?;
    Ok(count > 0)
}

pub fn update_buyer_totals(conn: &Connection, buyer_id: i32) -> Result<()> {
    conn.execute(
        "UPDATE buyer
         SET total = (SELECT SUM(total) FROM products_sell WHERE buyer = ?1),
             total_usd = (SELECT SUM(total_usd) FROM products_sell WHERE buyer = ?1)
         WHERE id = ?1",
        [buyer_id],
    )?;
    Ok(())
} */
