use derive_getters::Getters;
use diesel::prelude::*;
use serde::Serialize;

use crate::store::schema;
use crate::utils::get_timestamp;

use super::tag::Tag;

use super::NEW_ENTITY_ID;

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Debug, Getters)]
#[diesel(belongs_to(Tag, foreign_key=product_type))]
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
