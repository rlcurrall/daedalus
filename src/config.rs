use std::time::Duration;

use config::{Config, File};
use glob::glob;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationSeconds};

use crate::result::{AppError, Result};

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatabaseSettings {
    pub url: String,
    /// The maximum number of connections allowed in the pool.
    pub max_connections: u32,
    /// The maximum lifetime of a connection in the pool.
    #[serde_as(as = "DurationSeconds<u64>")]
    pub idle_timeout: Duration,
    /// The maximum time to wait when acquiring a new connection.
    #[serde_as(as = "DurationSeconds<u64>")]
    pub connection_timeout: Duration,
    /// The number of threads to use for the connection pool.
    pub thread_pool_size: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerSettings {
    pub port: u16,
    pub workers: usize,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct SessionSettings {
    pub secret: String,
    #[serde_as(as = "DurationSeconds<u64>")]
    pub lifetime: Duration,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppSettings {
    pub debug: bool,
    pub name: String,
    pub database: DatabaseSettings,
    pub server: ServerSettings,
    pub session: SessionSettings,
}

impl AppSettings {
    pub fn new() -> Result<Self> {
        let settings = Config::builder()
            .add_source(
                glob("config/*.toml")
                    .map_err(|e| AppError::ServerError {
                        cause: e.to_string(),
                    })?
                    .filter_map(|path| match path {
                        Err(_) => None,
                        Ok(path) => match path.to_str() {
                            Some(path) => Some(File::with_name(path)),
                            None => None,
                        },
                    })
                    .collect::<Vec<_>>(),
            )
            .build()
            .map_err(|e| AppError::ServerError {
                cause: e.to_string(),
            })?;

        Ok(settings
            .try_deserialize()
            .map_err(|e| AppError::ServerError {
                cause: e.to_string(),
            })
            .unwrap())
    }
}
