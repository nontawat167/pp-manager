use crate::store::schema;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::skus)]
pub struct SKUModel {
    pub id: String,
    #[diesel(column_name = "createdAt")]
    pub created_at: String,
    #[diesel(column_name = "updatedAt")]
    pub updated_at: String,
    pub name: String,
    pub price: i32,
    #[diesel(column_name = "productType")]
    pub product_type: String,
}
