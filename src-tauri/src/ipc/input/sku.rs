use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateSkuInput {
    pub name: String,
    pub price: i32,
    pub product_type: String,
}
#[derive(Deserialize, Serialize)]
pub struct SearchSkusInput {
    pub name: Option<String>,
    pub price: Option<i32>,
    pub product_type: Option<String>,

    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order_by: Option<String>,
}
