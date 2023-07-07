use super::input::SearchOperator;

pub struct SkuSearchInput {
    pub name: Option<SearchOperator<String>>,
    pub price: Option<SearchOperator<i32>>,
    pub product_type: Option<SearchOperator<String>>,

    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub order_by: Option<SkuOrderBy>,
}

impl Default for SkuSearchInput {
    fn default() -> Self {
        Self {
            name: None,
            price: None,
            product_type: None,

            page: None,
            per_page: None,
            order_by: None,
        }
    }
}

pub enum SkuOrderBy {
    CreatedAt,
    UpdatedAt,
}
