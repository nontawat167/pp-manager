use crate::domain::tags::Tag;
use crate::store::Store;

use async_trait::async_trait;
use sea_query::{Query, SqliteQueryBuilder};
use sea_query_binder::SqlxBinder;

use crate::domain::tag::{NewTag, TagsIden};

use crate::error::Result;
use std::sync::Arc;

pub struct TagRepository {
    store: Arc<Store>,
}

impl TagRepository {
    pub fn new(store: Store) -> Self {
        Self {
            store: Arc::new(store),
        }
    }

    pub async fn find_all(&self) -> Result<Vec<NewTag>> {
        let conn = self.store.connection();

        let mut q = Query::select();
        q.columns([
            TagsIden::Id,
            TagsIden::Name,
            TagsIden::Kind,
            TagsIden::Color,
        ])
        .from(TagsIden::Table);

        // q.and_where(Expr::col(TagsIden::Name).like("ยาง"));

        let (sql, values) = q.build_sqlx(SqliteQueryBuilder);

        let tags = sqlx::query_as_with::<_, NewTag, _>(&sql, values)
            .fetch_all(&*conn)
            .await
            .unwrap();

        Ok(tags)
    }
}

#[async_trait]
pub trait TagRepository1: Send + Sync  {
    async fn find_all(&self) -> Result<Vec<Tag>>;
}

