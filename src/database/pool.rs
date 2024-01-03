use diesel::r2d2;

use super::{DatabaseSettings, DbConnection, DbPool};

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

    fn build_pool(settings: &DatabaseSettings) -> DbPool {
        let manager = r2d2::ConnectionManager::<DbConnection>::new(&settings.database_url);
        let mut builder = r2d2::Pool::builder();

        if let Some(max_connections) = settings.max_connections {
            builder = builder.max_size(max_connections);
        }

        if let Some(idle_timeout) = settings.idle_timeout {
            builder = builder.idle_timeout(Some(idle_timeout));
        }

        if let Some(connection_timeout) = settings.connection_timeout {
            builder = builder.connection_timeout(connection_timeout);
        }

        builder.build(manager).expect("Failed to create pool.")
    }
}
