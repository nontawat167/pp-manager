use crate::error::Result;

pub trait MigrationEngine {
    fn migrate(&self) -> Result<()>;
}

pub struct Migrator {}

impl Migrator {
    pub fn new() -> Self {
        Migrator{}
    }

    pub fn run_migrations(&self, engine: impl MigrationEngine) -> Result<()> {
        engine.migrate()
    }
}
