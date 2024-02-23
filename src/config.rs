use std::{collections::HashMap, fmt::Display, path::PathBuf, str::FromStr, time::Duration};

use config::{Config as RustConfig, Environment, File};
use serde::{Deserialize, Serialize, Serializer};
use serde_with::{serde_as, DurationSeconds};

use crate::models::defaults::{
    default_bool, default_duration, default_u16, default_u32, default_usize,
};
use crate::result::{AppError, Result};

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JwtSettings {
    /// The path to the public key used to verify JWT tokens.
    /// This field is required.
    pub pub_key: PathBuf,

    /// The path to the private key used to sign JWT tokens.
    /// This field is required.
    pub priv_key: PathBuf,

    /// Lifetime of the JWT token in seconds.
    /// The default is 3600 seconds.
    #[serde_as(as = "DurationSeconds<u64>")]
    #[serde(default = "default_duration::<3600>")]
    pub lifetime: Duration,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatabaseSettings {
    /// The URL of the database.
    /// This field is required.
    #[serde(serialize_with = "obfuscate_string")]
    pub url: String,

    /// The maximum number of connections allowed in the pool.
    /// The default is 10.
    #[serde(default = "default_u32::<10>")]
    pub max_connections: u32,

    /// The maximum lifetime of a connection in the pool.
    /// The default is 600 seconds.
    #[serde_as(as = "DurationSeconds<u64>")]
    #[serde(default = "default_duration::<600>")]
    pub idle_timeout: Duration,

    /// The maximum time to wait when acquiring a new connection.
    /// The default is 30 seconds.
    #[serde_as(as = "DurationSeconds<u64>")]
    #[serde(default = "default_duration::<30>")]
    pub connection_timeout: Duration,

    /// The number of threads to use for the connection pool.
    /// The default is 3.
    #[serde(default = "default_u32::<3>")]
    pub thread_pool_size: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum LogFormat {
    Json,
    Pretty,
    Compact,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LogSettings {
    /// The level of the log.
    /// The default is "info".
    #[serde(default = "default_log_level")]
    pub level: LogLevel,

    /// The format of the log.
    /// The default is "json".
    #[serde(default = "default_log_format")]
    pub format: LogFormat,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerSettings {
    /// The port to bind the server to.
    /// The default is 8080.
    #[serde(default = "default_u16::<8080>")]
    pub port: u16,

    /// The number of worker threads to use.
    /// The default is 4.
    #[serde(default = "default_usize::<4>")]
    pub workers: usize,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SessionSettings {
    /// The secret used to sign the session cookie.
    /// This field is required.
    #[serde(serialize_with = "obfuscate_string")]
    pub secret: String,

    /// If the cookie should be secure.
    /// The default is true.
    #[serde(default = "default_bool::<true>")]
    pub secure: bool,

    /// The maximum lifetime of a session.
    /// The default is 7200 seconds.
    #[serde_as(as = "DurationSeconds<u64>")]
    #[serde(default = "default_duration::<7200>")]
    pub lifetime: Duration,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppSettings {
    /// The version of the application.
    /// This field is required.
    pub version: String,

    /// The name of the application.
    #[serde(default = "default_application_name")]
    pub name: String,

    /// If the application is in debug mode.
    /// The default is false.
    #[serde(default = "default_bool::<false>")]
    pub debug: bool,

    /// The settings for JWT.
    pub jwt: JwtSettings,

    /// The settings for the database.
    pub database: DatabaseSettings,

    /// The settings for logging and tracing.
    #[serde(default)]
    pub log: LogSettings,

    /// The settings for the server.
    #[serde(default)]
    pub server: ServerSettings,

    /// The settings for the session.
    pub session: SessionSettings,
}

#[derive(Debug)]
pub struct Config {
    version: String,
    overrides: HashMap<String, String>,
}

impl Config {
    pub fn new(version: String) -> Self {
        Self {
            version,
            overrides: HashMap::new(),
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

    pub fn set_name(mut self, name: String) -> Self {
        self.overrides.insert("name".to_string(), name.to_string());
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

    pub fn set_session_secret(mut self, secret: String) -> Self {
        self.overrides
            .insert("session.secret".to_string(), secret.to_string());
        self
    }

    pub fn set_session_lifetime(mut self, lifetime: u64) -> Self {
        self.overrides
            .insert("session.lifetime".to_string(), lifetime.to_string());
        self
    }

    pub fn set_session_secure(mut self, secure: bool) -> Self {
        self.overrides
            .insert("session.secure".to_string(), secure.to_string());
        self
    }

    pub fn parse(self) -> Result<AppSettings> {
        let mut builder = RustConfig::builder()
            .add_source(
                File::with_name("daedalus.toml")
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

        Ok(builder.build()?.try_deserialize()?)
    }
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            port: 8080,
            workers: 4,
        }
    }
}

impl Default for LogSettings {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            format: LogFormat::Json,
        }
    }
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

impl clap::ValueEnum for LogLevel {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            LogLevel::Error => clap::builder::PossibleValue::new("error"),
            LogLevel::Warn => clap::builder::PossibleValue::new("warn"),
            LogLevel::Info => clap::builder::PossibleValue::new("info"),
            LogLevel::Debug => clap::builder::PossibleValue::new("debug"),
            LogLevel::Trace => clap::builder::PossibleValue::new("trace"),
        })
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[
            LogLevel::Error,
            LogLevel::Warn,
            LogLevel::Info,
            LogLevel::Debug,
            LogLevel::Trace,
        ]
    }
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

impl clap::ValueEnum for LogFormat {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            LogFormat::Json => clap::builder::PossibleValue::new("json"),
            LogFormat::Pretty => clap::builder::PossibleValue::new("pretty"),
            LogFormat::Compact => clap::builder::PossibleValue::new("compact"),
        })
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[LogFormat::Json, LogFormat::Pretty, LogFormat::Compact]
    }
}

fn default_application_name() -> String {
    String::from("Server")
}

fn default_log_level() -> LogLevel {
    LogLevel::Info
}

fn default_log_format() -> LogFormat {
    LogFormat::Json
}

fn obfuscate_string<T, S>(value: T, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: Serializer,
{
    let mut obfuscated = String::new();
    value.as_ref().iter().for_each(|_| obfuscated.push('*'));

    serializer.serialize_str(&obfuscated)
}
