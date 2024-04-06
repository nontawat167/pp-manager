use derive_getters::Getters;
use diesel::prelude::*;
use sea_query::Iden;
use serde::Serialize;

use crate::store::schema;

use super::entity::NEW_ENTITY_ID;

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

#[derive(Iden)]
#[iden = "tags"]
pub enum TagsIden {
    Table,
    #[iden = "id"]
    Id,
    #[iden = "name"]
    Name,
    #[iden = "kind"]
    Kind,
    #[iden = "color"]
    Color,
}

// impl Iden for TagsIden {
//     fn unquoted(&self, s: &mut dyn std::fmt::Write) {
//         write!(
//             s,
//             "{}",
//             match self {
//                 Self::Table => "tags",
//                 Self::Id => "id",
//                 Self::Name => "name",
//                 Self::Kind => "kind",
//                 Self::Color => "color",
//             }
//         )
//         .unwrap();
//     }
// }

#[derive(Serialize, sqlx::FromRow)]
pub struct NewTag {
    id: String,
    name: String,
    kind: String,
    color: String,
}
