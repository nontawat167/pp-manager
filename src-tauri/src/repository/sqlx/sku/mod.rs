use async_trait::async_trait;
use sea_query::{Query, SqliteQueryBuilder};
use sea_query_binder::SqlxBinder;
use sqlx::{Pool, Sqlite};

use crate::domain::skus::Sku;
use crate::repository::sku::SkuRepository;
use crate::Result;

use self::entity::{SqlxSku, SqlxSkuIden};

use super::establish_connection;

pub mod entity;

pub struct SqlxSkuRepository {
    url: String,
}

impl SqlxSkuRepository {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    async fn connect(&self) -> Pool<Sqlite> {
        establish_connection(self.url.clone()).await
    }
}

#[async_trait]
impl SkuRepository for SqlxSkuRepository {
    async fn find(&self) -> Result<(i32, Vec<Sku>)> {
        let conn = self.connect().await;

        let mut q = Query::select();
        q.columns([
            SqlxSkuIden::Id,
            SqlxSkuIden::CreatedAt,
            SqlxSkuIden::UpdatedAt,
            SqlxSkuIden::DeletedAt,
            SqlxSkuIden::Name,
            SqlxSkuIden::Price,
            SqlxSkuIden::ProductType,
        ])
        .from(SqlxSkuIden::Table);

        let (sql, values) = q.build_sqlx(SqliteQueryBuilder);

        let skus: Vec<Sku> = sqlx::query_as_with::<_, SqlxSku, _>(&sql, values)
            .fetch_all(&conn)
            .await
            .unwrap()
            .into_iter()
            .map(|sku| sku.into())
            .collect();

        let qqq = format!(
            "SELECT COUNT(*) FROM ({}) AS t",
            q.to_string(SqliteQueryBuilder)
        );

        let count: i32 = sqlx::query_scalar(&qqq).fetch_one(&conn).await.unwrap();

        Ok((count, skus))
    }
}
