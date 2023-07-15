use super::input::SearchOperator;

pub struct TagSearchInput {
    pub kind: Option<SearchOperator<String>>,

    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order_by: Option<TagOrderBy>,
}

impl Default for TagSearchInput {
    fn default() -> Self {
        Self {
            kind: None,

            page: None,
            per_page: None,
            order_by: None,
        }
    }
}

pub enum SqlOrder {
    ASC,
    DESC,
}

pub enum TagOrderBy {
    Name(SqlOrder),
}
