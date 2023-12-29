use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct BuyerRecord {
    pub id: i64,
    pub category_type: String,
    pub dni: i64,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PresellItems {
    pub codebar: i64,
    pub price: i64,
    pub usd_value: i64,
    pub amount: i64,
    pub total: i64,
    pub total_usd: i64,
    pub buyer: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SellResult {
    pub success: Result<ItemForSell, String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemForSell {
    pub id: i64,
    pub codebar: i64,
    pub name: String,
    pub amount: i64,
    pub price: i64,
    pub usd_value: i64,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowItemForSell {
    pub id: i64,
    pub codebar: i64,
    pub amount: i64,
    pub price: i64,
    pub usd_value: i64,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseStatus {
    pub success: bool,
    pub error_message: Option<String>,
}
