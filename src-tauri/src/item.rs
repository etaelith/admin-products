use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct Item {
    pub id: i64,
    pub codebar: i64,
    pub name: String,
    pub stock: i64,
    pub price: i64,
    pub category: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BuyerRecord {
    pub id: i64,
    pub category_type: String,
    pub total: i64,
    pub total_usd: i64,
}
#[derive(Debug, Serialize)]
pub struct SaleItem {
    pub codebar: i64,
    pub amount: i64,
    pub price: i64,
    pub usd_value: i64,
    pub buyer: i64,
}
