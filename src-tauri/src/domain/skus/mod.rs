use serde::Serialize;

use crate::{repository::sqlx::sku::entity::SqlxSku, utils::get_timestamp};

use super::entity::NEW_ENTITY_ID;

pub mod usecase;

#[derive(Serialize, Debug)]
pub struct Sku {
    id: String,
    created_at: String,
    updated_at: String,
    deleted_at: Option<String>,
    name: String,
    price: i32,
    product_type: String,
}

impl Sku {
    pub fn new(name: String, price: i32, product_type: String) -> Self {
        let id = String::from(NEW_ENTITY_ID);
        let timestamp = get_timestamp();
        Self {
            id,
            created_at: timestamp.clone(),
            updated_at: timestamp,
            deleted_at: None,
            name,
            price,
            product_type,
        }
    }
}

impl From<SqlxSku> for Sku {
    fn from(value: SqlxSku) -> Self {
        Self {
            id: value.id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
            name: value.name,
            price: value.price,
            product_type: value.product_type,
        }
    }
}
