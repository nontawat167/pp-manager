use sea_query::Iden;
use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
pub struct SqlxSku {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub name: String,
    pub price: i32,
    pub product_type: String,
}

#[derive(Iden)]
#[iden = "skus"]
pub enum SqlxSkuIden {
    Table,
    #[iden = "id"]
    Id,
    #[iden = "created_at"]
    CreatedAt,
    #[iden = "updated_at"]
    UpdatedAt,
    #[iden = "deleted_at"]
    DeletedAt,
    #[iden = "name"]
    Name,
    #[iden = "price"]
    Price,
    #[iden = "product_type"]
    ProductType,
}
