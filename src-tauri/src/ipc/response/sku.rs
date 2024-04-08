use serde::Serialize;

use crate::domain::sku::Sku;
use crate::domain::skus::Sku as DomainSku;

#[derive(Serialize, Debug)]
pub struct SkuSearchResponse {
    pub total: i32,
    pub skus: Vec<Sku>,
}

#[derive(Serialize, Debug)]
pub struct SkuSearchResponse2 {
    pub total: i32,
    pub skus: Vec<DomainSku>,
}

impl From<(i32, Vec<DomainSku>)> for SkuSearchResponse2 {
    fn from(value: (i32, Vec<DomainSku>)) -> Self {
        Self {
            total: value.0,
            skus: value.1,
        }
    }
}
