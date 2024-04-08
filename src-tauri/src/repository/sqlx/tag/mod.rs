use async_trait::async_trait;
use sea_query::{Query, SqliteQueryBuilder};
use sea_query_binder::SqlxBinder;
use sqlx::{Pool, Sqlite};

use crate::Result;
use crate::{domain::tags::Tag, repository::tag::TagRepository1};

use self::entity::{SqlxTag, SqlxTagIden};

use super::establish_connection;

pub mod entity;

pub struct SqlxTagRepository {
    url: String,
}

impl SqlxTagRepository {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    async fn connect(&self) -> Pool<Sqlite> {
        establish_connection(self.url.clone()).await
    }
}

#[async_trait]
impl TagRepository1 for SqlxTagRepository {
    async fn find_all(&self) -> Result<Vec<Tag>> {
        let conn = self.connect().await;

        let mut q = Query::select();
        q.columns([
            SqlxTagIden::Id,
            SqlxTagIden::Name,
            SqlxTagIden::Kind,
            SqlxTagIden::Color,
        ])
        .from(SqlxTagIden::Table);

        // q.and_where(Expr::col(TagsIden::Name).like("ยาง"));

        let (sql, values) = q.build_sqlx(SqliteQueryBuilder);

        let tags: Vec<Tag> = sqlx::query_as_with::<_, SqlxTag, _>(&sql, values)
            .fetch_all(&conn)
            .await
            .unwrap()
            .into_iter()
            .map(|tag| tag.into())
            .collect();

        Ok(tags)
    }
}
