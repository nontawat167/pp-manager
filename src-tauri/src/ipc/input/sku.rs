use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateSkuInput {
    pub name: String,
    pub price: i32,
    pub product_type: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchSkusInput {
    pub name: Option<String>,
    pub price: Option<i32>,
    pub product_type: Option<String>,

    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Types {
    create_sku_input: CreateSkuInput,
    search_skus_input: SearchSkusInput,
}
