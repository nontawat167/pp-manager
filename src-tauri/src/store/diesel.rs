use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::error::Result;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

use super::migrator::MigrationEngine;

pub struct DieselMigrationEngine {
    url: String,
}

impl DieselMigrationEngine {
    pub fn new(url: String) -> Self {
        DieselMigrationEngine { url }
    }

    pub fn establish_connection(&self) -> SqliteConnection {
        //diesel migration run --database-url=D:\\test.db
        // let database_url = "D:\\test.db";
        SqliteConnection::establish(&self.url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", &self.url))
    }

    pub fn run_migrations(&self, connection: &mut impl MigrationHarness<Sqlite>) -> Result<()> {
        let versions = connection.run_pending_migrations(MIGRATIONS).unwrap();
        println!("Running migration...");
        for version in versions.into_iter() {
            println!("migrated: {}", version)
        }

        Ok(())
    }
}

impl MigrationEngine for DieselMigrationEngine {
    fn migrate(&self) -> Result<()> {
        let mut conn = self.establish_connection();
        self.run_migrations(&mut conn)
    }
}
