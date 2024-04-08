use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

pub mod tag;
pub mod sku;

// url: "sqlite://D:\\test.db"
pub(crate) async fn establish_connection(url: String) -> Pool<Sqlite> {
    SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&url)
        .await
        .unwrap()
}