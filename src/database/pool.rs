use diesel::r2d2;

use crate::config::DatabaseSettings;
use crate::result::{AppError, Result};

use super::{DbConnection, DbPool, PooledConnection};

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

    pub fn get_connection(&self) -> Result<PooledConnection> {
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
}
