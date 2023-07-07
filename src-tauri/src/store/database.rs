use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::error;
use error::Result;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub struct DatabaseContext {
    url: String,
}

impl DatabaseContext {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) -> Result<()> {
        let versions = connection.run_pending_migrations(MIGRATIONS).unwrap();
        println!("Running migration...");
        for version in versions.into_iter() {
            println!("migrated: {}", version)
        }

        Ok(())
    }

    pub fn establish_connection(&self) -> SqliteConnection {
        //diesel migration run --database-url=D:\\test.db
        // let database_url = "D:\\test.db";
        let database_url = self.url.clone();
        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}
