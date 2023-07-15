use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateTagInput {
    pub name: String,
    pub kind: String,
    pub color: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchTagsInput {
    pub kind: Option<String>,

    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order: Option<String>,
}
