use derive_getters::Getters;
use diesel::prelude::*;
use serde::Serialize;

use crate::store::schema;

use super::NEW_ENTITY_ID;

#[derive(Queryable, Selectable, Identifiable, Serialize, Debug, Getters)]
#[diesel(table_name = schema::tags)]
pub struct Tag {
    id: String,
    name: String,
    kind: String,
    color: String,
}

impl Tag {
    pub fn new(name: String, kind: String, color: String) -> Self {
        let id = String::from(NEW_ENTITY_ID);
        Self {
            id,
            name,
            kind,
            color,
        }
    }
}
