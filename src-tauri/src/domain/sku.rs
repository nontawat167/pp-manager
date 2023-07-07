use derive_getters::Getters;
use diesel::prelude::*;
use serde::Serialize;

use crate::repository::schema;
use crate::utils::get_timestamp;

#[derive(Queryable, Selectable, Serialize, Debug, Getters)]
#[diesel(table_name = schema::skus)]
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
        let id = String::from("new-entity");
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
