use serde::Serialize;

use crate::domain::sku::Sku;

#[derive(Serialize, Debug)]
pub struct SkuSearchResponse {
    pub total: i32,
    pub skus: Vec<Sku>,
}
