use std::time::Duration;

use diesel::{r2d2, PgConnection};

mod migrator;
mod pool;
pub mod schema;

pub use migrator::Migrator;
pub use pool::PoolManager;

pub type DB = diesel::pg::Pg;
pub type DbConnection = PgConnection;
pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<DbConnection>>;
pub type PooledConnection = r2d2::PooledConnection<r2d2::ConnectionManager<DbConnection>>;

#[derive(Clone, Debug)]
pub struct DatabaseSettings {
    pub database_url: String,
    /// The maximum number of connections allowed in the pool.
    /// Defaults to 10.
    pub max_connections: Option<u32>,
    /// The maximum lifetime of a connection in the pool.
    /// Defaults to 10 minutes.
    pub idle_timeout: Option<Duration>,
    /// The maximum time to wait when acquiring a new connection.
    /// Defaults to 30 seconds.
    pub connection_timeout: Option<Duration>,
    /// The number of threads to use for the connection pool.
    /// Defaults to 3.
    pub thread_pool_size: Option<u32>,
}

impl DatabaseSettings {
    pub fn new(database_url: String) -> Self {
        Self {
            database_url,
            max_connections: None,
            idle_timeout: None,
            connection_timeout: None,
            thread_pool_size: None,
        }
    }
}
