use super::StoreConnection;
use sqlx::sqlite::SqlitePoolOptions;

pub async fn establish_connection() -> StoreConnection {
    SqlitePoolOptions::new()
        .max_connections(10)
        .connect("sqlite://D:\\test.db")
        .await
        .unwrap()
}
