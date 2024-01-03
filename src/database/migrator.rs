use std::error::Error;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use super::{DbPool, PooledConnection, DB};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct Migrator {
    pool: DbPool,
}

impl Migrator {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let mut conn = self.get_connection()?;

        Self::inner_run(&mut conn)?;

        Ok(())
    }

    pub fn get_connection(
        &self,
    ) -> Result<PooledConnection, Box<dyn Error + Send + Sync + 'static>> {
        Ok(self.pool.get()?)
    }

    fn inner_run(
        connection: &mut impl MigrationHarness<DB>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        connection.run_pending_migrations(MIGRATIONS)?;

        Ok(())
    }
}
