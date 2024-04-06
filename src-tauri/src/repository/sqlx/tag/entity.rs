use sea_query::Iden;
use serde::Serialize;

use crate::domain::tags::Tag;

#[derive(Serialize, sqlx::FromRow)]
pub struct SqlxTag {
    id: String,
    name: String,
    kind: String,
    color: String,
}

#[derive(Iden)]
#[iden = "tags"]
pub enum SqlxTagIden {
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

impl Into<Tag> for SqlxTag {
    fn into(self) -> Tag {
        Tag::new(
            Some(self.id.to_owned()),
            self.name.to_owned(),
            self.kind.to_owned(),
            self.color.to_owned(),
        )
    }
}
