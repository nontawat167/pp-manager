use serde::Serialize;

use super::entity::{Entity, NEW_ENTITY_ID};

pub mod usecase;

#[derive(Serialize)]
pub struct Tag {
    id: String,
    name: String,
    kind: String,
    color: String,
}

impl Tag {
    pub fn new(id: Option<String>, name: String, kind: String, color: String) -> Self {
        let id = match id {
            Some(id) => id,
            None => String::from(NEW_ENTITY_ID),
        };

        Self {
            id,
            name,
            kind,
            color,
        }
    }
}

impl Entity for Tag {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}
