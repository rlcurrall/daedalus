use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use console::style;
use tracing_subscriber::{self, filter::Targets, layer::SubscriberExt, util::SubscriberInitExt};

use daedalus::config::{AppSettings, ConfigBuilder, LogFormat, LogLevel};
use daedalus::result::{AppError, Result};
use daedalus::server;

#[actix_web::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let app_settings = match parse_args() {
        Ok(settings) => settings,
        Err(e) => {
            print_error(e);
            std::process::exit(1);
        }
    };

    tracing_init(&app_settings);

    server::start(app_settings).await
}

fn tracing_init(settings: &AppSettings) {
    let log_level: tracing::Level = settings.log.level.clone().into();
    let base_level = match settings.debug {
        true => log_level,
        false => tracing::Level::ERROR,
    };

    let registry = tracing_subscriber::registry().with(
        Targets::new()
            .with_target("daedalus", log_level)
            .with_default(base_level),
    );

    match settings.log.format {
        LogFormat::Json => registry
            .with(tracing_subscriber::fmt::layer().json())
            .init(),
        LogFormat::Pretty => registry
            .with(tracing_subscriber::fmt::layer().pretty())
            .init(),
        LogFormat::Compact => registry
            .with(tracing_subscriber::fmt::layer().compact())
            .init(),
    }
}

fn parse_args() -> Result<AppSettings> {
    let version = env!("BUILD_ID");

    let serve_cmd = Serve::parse();

    let mut config = ConfigBuilder::new(version.into());

    if let Some(name) = serve_cmd.name {
        config = config.set_name(name.to_string());
    }
    if let Some(debug) = serve_cmd.debug {
        config = config.set_debug(debug);
    }
    if let Some(db_url) = serve_cmd.db_url {
        config = config.set_db_url(db_url.to_string());
    }
    if let Some(db_max_conns) = serve_cmd.db_max_conns {
        config = config.set_db_max_connections(db_max_conns);
    }
    if let Some(db_conn_timeout) = serve_cmd.db_conn_timeout {
        config = config.set_db_conn_timeout(db_conn_timeout);
    }
    if let Some(db_idle_timeout) = serve_cmd.db_idle_timeout {
        config = config.set_db_idle_timeout(db_idle_timeout);
    }
    if let Some(db_thread_size) = serve_cmd.db_thread_size {
        config = config.set_db_thread_size(db_thread_size);
    }
    if let Some(log_level) = serve_cmd.log_level {
        config = config.set_log_level(log_level.to_owned());
    }
    if let Some(log_format) = serve_cmd.log_format {
        config = config.set_log_format(log_format.to_owned());
    }
    if let Some(jwt_pub_key) = serve_cmd.jwt_pub_key {
        config = config.set_jwt_pub_key(
            &PathBuf::from_str(&jwt_pub_key).map_err(|e| AppError::server_error(e))?,
        );
    }
    if let Some(jwt_priv_key) = serve_cmd.jwt_priv_key {
        config = config.set_jwt_priv_key(
            &PathBuf::from_str(&jwt_priv_key).map_err(|e| AppError::server_error(e))?,
        );
    }
    if let Some(jwt_lifetime) = serve_cmd.jwt_lifetime {
        config = config.set_jwt_lifetime(jwt_lifetime);
    }
    if let Some(server_port) = serve_cmd.server_port {
        config = config.set_server_port(server_port);
    }
    if let Some(server_workers) = serve_cmd.server_workers {
        config = config.set_server_workers(server_workers);
    }
    if let Some(session_secret) = serve_cmd.session_secret {
        config = config.set_session_secret(session_secret);
    }
    if let Some(session_secure) = serve_cmd.session_secure {
        config = config.set_session_secure(session_secure);
    }
    if let Some(session_lifetime) = serve_cmd.session_lifetime {
        config = config.set_session_lifetime(session_lifetime);
    }

    config.parse()
}

const ABOUT: &str = r#"
______               _       _
|  _  \             | |     | |
| | | |__ _  ___  __| | __ _| |_   _ ___
| | | / _` |/ _ \/ _` |/ _` | | | | / __|
| |/ / (_| |  __/ (_| | (_| | | |_| \__ \
|___/ \__,_|\___|\__,_|\__,_|_|\__,_|___/

Daedalus application server
"#;

#[derive(clap::Parser)]
#[clap(name = "daedalus", about = ABOUT)]
pub struct Serve {
    #[clap(short = 'n', long)]
    pub name: Option<String>,
    #[clap(short = 'd', long)]
    pub debug: Option<bool>,
    #[clap(short = 'D', long)]
    pub db_url: Option<String>,
    #[clap(long)]
    pub db_max_conns: Option<u32>,
    #[clap(long)]
    pub db_conn_timeout: Option<u64>,
    #[clap(long)]
    pub db_idle_timeout: Option<u64>,
    #[clap(long)]
    pub db_thread_size: Option<u32>,
    #[clap(long)]
    pub log_level: Option<LogLevel>,
    #[clap(long)]
    pub log_format: Option<LogFormat>,
    #[clap(long)]
    pub jwt_pub_key: Option<String>,
    #[clap(long)]
    pub jwt_priv_key: Option<String>,
    #[clap(long)]
    pub jwt_lifetime: Option<u64>,
    #[clap(short = 'p', long)]
    pub server_port: Option<u16>,
    #[clap(short = 'w', long)]
    pub server_workers: Option<usize>,
    #[clap(long)]
    pub session_secret: Option<String>,
    #[clap(long)]
    pub session_secure: Option<bool>,
    #[clap(long)]
    pub session_lifetime: Option<u64>,
}

fn print_error(e: AppError) {
    eprintln!(
        "{}\n\n\t{}\n",
        style("Failed to parse configuration:").bold().red(),
        e
    );
}
