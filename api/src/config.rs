use std::{collections::HashMap, fmt::Display, path::PathBuf, str::FromStr, time::Duration};

use clap::ValueEnum;
use config::{Config as RustConfig, Environment, File};
use serde::{Deserialize, Serialize, Serializer};
use serde_with::{serde_as, DurationSeconds};

use crate::result::{AppError, Result};

// region: JWT settings
#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
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

// region: Database settings
#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatabaseSettings {
    /// The URL of the database.
    /// This field is required.
    #[serde(serialize_with = "obfuscate_string")]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LogSettings {
    /// The level of the log.
    /// The default is "info".
    pub level: LogLevel,

    /// The format of the log.
    /// The default is "json".
    pub format: LogFormat,
}
// endregion

// region: Server settings
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerSettings {
    /// The port to bind the server to.
    /// The default is 8080.
    pub port: u16,

    /// The number of worker threads to use.
    /// The default is 4.
    pub workers: usize,
}
// endregion

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Debug)]
pub struct ConfigBuilder {
    version: String,
    overrides: HashMap<String, String>,
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

    pub fn with_overrides(mut self, overrides: HashMap<String, String>) -> Self {
        self.overrides = overrides;
        self
    }

    pub fn set_debug(mut self, debug: bool) -> Self {
        self.overrides
            .insert("debug".to_string(), debug.to_string());
        self
    }

    pub fn set_db_url(mut self, url: String) -> Self {
        self.overrides
            .insert("database.url".to_string(), url.to_string());
        self
    }

    pub fn set_db_max_connections(mut self, max_connections: u32) -> Self {
        self.overrides.insert(
            "database.max_connections".to_string(),
            max_connections.to_string(),
        );
        self
    }

    pub fn set_db_idle_timeout(mut self, idle_timeout: u64) -> Self {
        self.overrides.insert(
            "database.idle_timeout".to_string(),
            idle_timeout.to_string(),
        );
        self
    }

    pub fn set_db_conn_timeout(mut self, conn_timeout: u64) -> Self {
        self.overrides.insert(
            "database.connection_timeout".to_string(),
            conn_timeout.to_string(),
        );
        self
    }

    pub fn set_db_thread_size(mut self, thread_size: u32) -> Self {
        self.overrides.insert(
            "database.thread_pool_size".to_string(),
            thread_size.to_string(),
        );
        self
    }

    pub fn set_jwt_pub_key(mut self, pub_key: &PathBuf) -> Self {
        self.overrides.insert(
            "jwt.pub_key".to_string(),
            pub_key.to_string_lossy().to_string(),
        );
        self
    }

    pub fn set_jwt_priv_key(mut self, priv_key: &PathBuf) -> Self {
        self.overrides.insert(
            "jwt.priv_key".to_string(),
            priv_key.to_string_lossy().to_string(),
        );
        self
    }

    pub fn set_jwt_lifetime(mut self, lifetime: u64) -> Self {
        self.overrides
            .insert("jwt.lifetime".to_string(), lifetime.to_string());
        self
    }

    pub fn set_log_level(mut self, level: LogLevel) -> Self {
        self.overrides
            .insert("log.level".to_string(), level.to_string());
        self
    }

    pub fn set_log_format(mut self, format: LogFormat) -> Self {
        self.overrides
            .insert("log.format".to_string(), format.to_string());
        self
    }

    pub fn set_server_port(mut self, port: u16) -> Self {
        self.overrides
            .insert("server.port".to_string(), port.to_string());
        self
    }

    pub fn set_server_workers(mut self, workers: usize) -> Self {
        self.overrides
            .insert("server.workers".to_string(), workers.to_string());
        self
    }

    pub fn parse(self) -> Result<AppSettings> {
        let mut builder = RustConfig::builder()
            .set_default("debug", false)?
            .set_default("jwt.pub_key", "./conf/public.pem")?
            .set_default("jwt.priv_key", "./conf/private.pem")?
            .set_default("jwt.lifetime", 3600)?
            .set_default("database.max_connections", 10)?
            .set_default("database.idle_timeout", 600)?
            .set_default("database.connection_timeout", 30)?
            .set_default("database.thread_pool_size", 3)?
            .set_default("log.level", LogLevel::default().to_string())?
            .set_default("log.format", LogFormat::default().to_string())?
            .set_default("server.port", 8080)?
            .set_default("server.workers", 4)?
            .add_source(
                File::with_name(&self.config_file)
                    .required(false)
                    .format(config::FileFormat::Toml),
            )
            .add_source(
                Environment::with_prefix("DA")
                    .separator("_")
                    .list_separator(" "),
            )
            .set_override("version", self.version)?;

        for (key, value) in self.overrides {
            builder = builder.set_override(&key, value)?;
        }

        Ok(builder
            .build()
            .map_err(|e| {
                println!("Oops #1 Error: {:?}", e);
                e
            })?
            .try_deserialize()
            .map_err(|e| {
                println!("Oops #2 Error: {:?}", e);
                e
            })?)
    }
}

fn obfuscate_string<T, S>(_: T, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: Serializer,
{
    serializer.serialize_str("**********")
}
