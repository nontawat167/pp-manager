use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::error;
pub use error::{Error, Result};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub struct Database {
    pub connection: SqliteConnection,
}

impl Database {
    pub fn new(url: String) -> Self {
        let mut conn = Self::establish_connection(url);
        let _ = Self::run_migrations(&mut conn);
        Self { connection: conn }
    }

    fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) -> Result<()> {
        let versions = connection.run_pending_migrations(MIGRATIONS).unwrap();
        println!("Running migration...");
        for version in versions.into_iter() {
            println!("migrated: {}", version)
        }

        Ok(())
    }

    fn establish_connection(database_url: String) -> SqliteConnection {
        //diesel migration run --database-url=D:\\test.db
        // let database_url = "D:\\test.db";
        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}
