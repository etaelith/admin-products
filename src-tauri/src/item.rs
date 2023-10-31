use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Item {
    pub id: i64,
    pub codebar: String,
    pub name: String,
    pub stock: i32,
    pub price: f64,
    pub category: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BuyerRecord {
    pub id: i64,
    pub category_type: String,
    pub total: f64,
    pub total_usd: f64,
}
