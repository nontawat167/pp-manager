pub mod database;
pub mod diesel;
pub mod migrator;
pub mod schema;
pub mod connection;


use std::sync::Arc;

use sqlx::{Pool, Sqlite};

type StoreConnection = Pool<Sqlite>;

pub struct Store {
    con: Arc<StoreConnection>,
}

impl Store {
    pub fn new(conn: StoreConnection) -> Self {
        Self {
            con: Arc::new(conn),
        }
    }

    pub fn connection(&self) -> Arc<StoreConnection> {
        self.con.clone()
    }
}
