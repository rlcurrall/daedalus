use std::{collections::HashMap, fmt::Display, path::PathBuf, str::FromStr, time::Duration};

use clap::ValueEnum;
use figment::{
    providers::{Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, DurationSeconds};

use crate::result::{AppError, Result};

#[derive(Clone, Debug, Deserialize)]
pub struct AppSettings {
    /// The version of the application.
    /// This field is required.
    pub version: String,

    /// If the application is in debug mode.
    /// The default is false.
    pub debug: bool,

    /// The settings for JWT.
    pub jwt: JwtSettings,

    /// The settings for the database.
    pub database: DatabaseSettings,

    /// The settings for logging and tracing.
    pub log: LogSettings,

    /// The settings for the server.
    pub server: ServerSettings,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseSettings {
    /// The URL of the database.
    /// This field is required.
    pub url: String,

    /// The maximum number of connections allowed in the pool.
    /// The default is 10.
    pub max_connections: u32,

    /// The maximum lifetime of a connection in the pool.
    /// The default is 600 seconds.
    #[serde_as(as = "DurationSeconds<u64>")]
    pub idle_timeout: Duration,

    /// The maximum time to wait when acquiring a new connection.
    /// The default is 30 seconds.
    #[serde_as(as = "DurationSeconds<u64>")]
    pub connection_timeout: Duration,

    /// The number of threads to use for the connection pool.
    /// The default is 3.
    pub thread_pool_size: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerSettings {
    /// The port to bind the server to.
    /// The default is 8080.
    pub port: u16,

    /// The number of worker threads to use.
    /// The default is 4.
    pub workers: usize,
}

// region: JWT settings

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
pub struct JwtSettings {
    /// The path to the public key used to verify JWT tokens.
    pub pub_key: PathBuf,

    /// The path to the private key used to sign JWT tokens.
    pub priv_key: PathBuf,

    /// Lifetime of the JWT token in seconds.
    /// The default is 3600 seconds.
    #[serde_as(as = "DurationSeconds<u64>")]
    pub lifetime: Duration,
}

// endregion

// region: Log settings

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, ValueEnum, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for tracing::Level {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
        }
    }
}

impl FromStr for LogLevel {
    type Err = AppError;

    fn from_str(s: &str) -> std::result::Result<LogLevel, Self::Err> {
        match s.to_lowercase() {
            s if s == "trace" => Ok(LogLevel::Trace),
            s if s == "debug" => Ok(LogLevel::Debug),
            s if s == "info" => Ok(LogLevel::Info),
            s if s == "warn" => Ok(LogLevel::Warn),
            s if s == "error" => Ok(LogLevel::Error),
            _ => Err(AppError::BadRequest {
                cause: format!("{} is not a valid log level", s),
            }),
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        };

        write!(f, "{}", value)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, ValueEnum, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    #[default]
    Json,
    Pretty,
    Compact,
}

impl FromStr for LogFormat {
    type Err = AppError;

    fn from_str(s: &str) -> std::result::Result<LogFormat, Self::Err> {
        match s.to_lowercase() {
            s if s == "json" => Ok(LogFormat::Json),
            s if s == "pretty" => Ok(LogFormat::Pretty),
            s if s == "compact" => Ok(LogFormat::Compact),
            _ => Err(AppError::BadRequest {
                cause: format!("{} is not a valid log format", s),
            }),
        }
    }
}

impl Display for LogFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            LogFormat::Json => "json",
            LogFormat::Pretty => "pretty",
            LogFormat::Compact => "compact",
        };

        write!(f, "{}", value)
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct LogSettings {
    /// The level of the log.
    /// The default is "info".
    pub level: LogLevel,

    /// The format of the log.
    /// The default is "json".
    pub format: LogFormat,
}

// endregion

// region: config builder

const DEBUG: &'static str = "debug";
const DB_URL: &'static str = "database.url";
const DB_MAX_CONNS: &'static str = "database.max_connections";
const DB_IDLE_TIMEOUT: &'static str = "database.idle_timeout";
const DB_CONN_TIMEOUT: &'static str = "database.connection_timeout";
const DB_THREAD_SIZE: &'static str = "database.thread_pool_size";
const JWT_PUB_KEY: &'static str = "jwt.pub_key";
const JWT_PRIV_KEY: &'static str = "jwt.priv_key";
const JWT_LIFETIME: &'static str = "jwt.lifetime";
const LOG_LEVEL: &'static str = "log.level";
const LOG_FORMAT: &'static str = "log.format";
const SERVER_PORT: &'static str = "server.port";
const SERVER_WORKERS: &'static str = "server.workers";

#[derive(Debug)]
pub struct ConfigBuilder {
    version: String,
    overrides: HashMap<String, Value>,
    config_file: String,
}

impl ConfigBuilder {
    pub fn new(version: String, config_file: Option<String>) -> Self {
        Self {
            version,
            overrides: HashMap::new(),
            config_file: config_file.unwrap_or_else(|| "daedalus.toml".to_string()),
        }
    }

    pub fn set_debug(mut self, debug: Option<bool>) -> Self {
        self.overrides.insert(DEBUG.into(), Value::from(debug));
        self
    }

    pub fn set_db_url(mut self, db_url: Option<String>) -> Self {
        self.overrides.insert(DB_URL.into(), Value::from(db_url));
        self
    }

    pub fn set_db_max_conns(mut self, db_max_conns: Option<u32>) -> Self {
        self.overrides
            .insert(DB_MAX_CONNS.into(), Value::from(db_max_conns));
        self
    }

    pub fn set_db_idle_timeout(mut self, db_idle_timeout: Option<u64>) -> Self {
        self.overrides
            .insert(DB_IDLE_TIMEOUT.into(), Value::from(db_idle_timeout));
        self
    }

    pub fn set_db_conn_timeout(mut self, db_conn_timeout: Option<u64>) -> Self {
        self.overrides
            .insert(DB_CONN_TIMEOUT.into(), Value::from(db_conn_timeout));
        self
    }

    pub fn set_db_thread_size(mut self, db_thread_size: Option<u32>) -> Self {
        self.overrides
            .insert(DB_THREAD_SIZE.into(), Value::from(db_thread_size));
        self
    }

    pub fn set_jwt_pub_key(mut self, jwt_pub_key: Option<String>) -> Self {
        self.overrides
            .insert(JWT_PUB_KEY.into(), Value::from(jwt_pub_key));
        self
    }

    pub fn set_jwt_priv_key(mut self, jwt_priv_key: Option<String>) -> Self {
        self.overrides
            .insert(JWT_PRIV_KEY.into(), Value::from(jwt_priv_key));
        self
    }

    pub fn set_jwt_lifetime(mut self, jwt_lifetime: Option<u64>) -> Self {
        self.overrides
            .insert(JWT_LIFETIME.into(), Value::from(jwt_lifetime));
        self
    }

    pub fn set_log_level(mut self, log_level: Option<LogLevel>) -> Self {
        self.overrides.insert(
            LOG_LEVEL.into(),
            Value::from(log_level.map(|l| l.to_string())),
        );
        self
    }

    pub fn set_log_format(mut self, log_format: Option<LogFormat>) -> Self {
        self.overrides.insert(
            LOG_FORMAT.into(),
            Value::from(log_format.map(|l| l.to_string())),
        );
        self
    }

    pub fn set_server_port(mut self, server_port: Option<u16>) -> Self {
        self.overrides
            .insert(SERVER_PORT.into(), Value::from(server_port));
        self
    }

    pub fn set_server_workers(mut self, server_workers: Option<usize>) -> Self {
        self.overrides
            .insert(SERVER_WORKERS.into(), Value::from(server_workers));
        self
    }

    pub fn parse(self) -> Result<AppSettings> {
        // Initialize with defaults
        let mut fig = Figment::new()
            .merge(Serialized::default("version", self.version.clone()))
            .merge(Serialized::default(DEBUG, false))
            .merge(Serialized::default(JWT_PUB_KEY, "./conf/public.pem"))
            .merge(Serialized::default(JWT_PRIV_KEY, "./conf/private.pem"))
            .merge(Serialized::default(JWT_LIFETIME, 3600))
            .merge(Serialized::default(DB_MAX_CONNS, 10))
            .merge(Serialized::default(DB_IDLE_TIMEOUT, 600))
            .merge(Serialized::default(DB_CONN_TIMEOUT, 30))
            .merge(Serialized::default(DB_THREAD_SIZE, 3))
            .merge(Serialized::default(LOG_LEVEL, LogLevel::default()))
            .merge(Serialized::default(LOG_FORMAT, LogFormat::default()))
            .merge(Serialized::default(SERVER_PORT, 8080))
            .merge(Serialized::default(SERVER_WORKERS, 4));

        // Add the config file source
        fig = fig.merge(Toml::file(self.config_file.clone()));

        // Add the overrides, skipping null values
        for (key, value) in self.overrides {
            if value.is_null() {
                continue;
            }
            fig = fig.merge(Serialized::default(&key, value));
        }

        fig.extract::<AppSettings>()
            .map_err(|e| match e.kind.clone() {
                figment::error::Kind::MissingField(k) => {
                    AppError::server_error(format!("Missing Field: {}.{}", e.path.join("."), k))
                }
                _ => AppError::server_error(format!("Error: {}", e)),
            })
    }
}

// endregion
