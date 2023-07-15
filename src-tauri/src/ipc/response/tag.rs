use serde::Serialize;

use crate::domain::tag::Tag;

#[derive(Serialize, Debug)]
pub struct TagSearchResponse {
    pub total: i32,
    pub tags: Vec<Tag>,
}
