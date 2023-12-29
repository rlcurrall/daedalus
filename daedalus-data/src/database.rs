use std::error::Error;
use std::time::Duration;

use diesel::{r2d2, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub type DB = diesel::pg::Pg;
pub type DbConnection = PgConnection;
pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<DbConnection>>;
pub type PooledConnection = r2d2::PooledConnection<r2d2::ConnectionManager<DbConnection>>;

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
