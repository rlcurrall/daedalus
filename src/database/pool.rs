use diesel::r2d2;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::config::DatabaseSettings;
use crate::result::{AppError, Result};

use super::{DbConnection, DbPool, PooledConnection};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[derive(Clone)]
pub struct PoolManager {
    pool: DbPool,
}

impl PoolManager {
    pub fn new(settings: &DatabaseSettings) -> Self {
        Self {
            pool: Self::build_pool(settings),
        }
    }

    pub fn get_pool(&self) -> DbPool {
        self.pool.clone()
    }

    pub fn get(&self) -> Result<PooledConnection> {
        self.pool.get().map_err(|e| AppError::ServerError {
            cause: e.to_string(),
        })
    }

    fn build_pool(settings: &DatabaseSettings) -> DbPool {
        let manager = r2d2::ConnectionManager::<DbConnection>::new(&settings.url);

        r2d2::Pool::builder()
            .max_size(settings.max_connections)
            .idle_timeout(Some(settings.idle_timeout))
            .connection_timeout(settings.connection_timeout)
            .build(manager)
            .expect("Failed to create pool.")
    }

    pub fn migrate(&mut self) -> std::result::Result<(), AppError> {
        let mut conn = self.get()?;
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| AppError::ServerError {
                cause: e.to_string(),
            })?;

        Ok(())
    }
}
