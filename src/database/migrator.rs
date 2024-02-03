use std::error::Error;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use super::PooledConnection;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct Migrator {
    conn: PooledConnection,
}

impl Migrator {
    pub fn new(conn: PooledConnection) -> Self {
        Self { conn }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.conn.run_pending_migrations(MIGRATIONS)?;

        Ok(())
    }
}
