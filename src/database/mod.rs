use diesel::{r2d2, PgConnection};

mod pool;
pub mod schema;

pub use pool::PoolManager;

pub type DB = diesel::pg::Pg;
pub type DbConnection = PgConnection;
pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<DbConnection>>;
pub type PooledConnection = r2d2::PooledConnection<r2d2::ConnectionManager<DbConnection>>;
