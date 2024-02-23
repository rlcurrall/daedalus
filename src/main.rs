use std::{path::PathBuf, str::FromStr};

use clap::{value_parser, Arg, ArgAction, Command};
use console::style;
use tracing_subscriber::{self, filter::Targets, layer::SubscriberExt, util::SubscriberInitExt};

use daedalus::{
    config::{AppSettings, Config, LogFormat, LogLevel},
    result::{AppError, Result},
    server,
};

#[actix_web::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let app_settings = match parse_args() {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!(
                "{}\n\n\t{}\n",
                style("Failed to parse configuration:").bold().red(),
                e
            );
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

    let matches = cli().get_matches();

    let mut config = Config::new(version.into());

    if let Some(name) = matches.get_one::<String>("name") {
        config = config.set_name(name.to_string());
    }
    if matches.get_flag("debug") {
        config = config.set_debug(true);
    }
    if let Some(db_url) = matches.get_one::<String>("db-url") {
        config = config.set_db_url(db_url.to_string());
    }
    if let Some(db_max_conns) = matches.get_one::<u32>("db-max-conns") {
        config = config.set_db_max_connections(*db_max_conns);
    }
    if let Some(db_conn_timeout) = matches.get_one::<u64>("db-conn-timeout") {
        config = config.set_db_conn_timeout(*db_conn_timeout);
    }
    if let Some(db_idle_timeout) = matches.get_one::<u64>("db-idle-timeout") {
        config = config.set_db_idle_timeout(*db_idle_timeout);
    }
    if let Some(db_thread_size) = matches.get_one::<u32>("db-thread-size") {
        config = config.set_db_thread_size(*db_thread_size);
    }
    if let Some(log_level) = matches.get_one::<LogLevel>("log-level") {
        config = config.set_log_level(log_level.to_owned());
    }
    if let Some(log_format) = matches.get_one::<LogFormat>("log-format") {
        config = config.set_log_format(log_format.to_owned());
    }
    if let Some(jwt_pub_key) = matches.get_one::<String>("jwt-pub-key") {
        config = config.set_jwt_pub_key(
            &PathBuf::from_str(&jwt_pub_key).map_err(|e| AppError::server_error(e))?,
        );
    }
    if let Some(jwt_priv_key) = matches.get_one::<String>("jwt-priv-key") {
        config = config.set_jwt_priv_key(
            &PathBuf::from_str(jwt_priv_key).map_err(|e| AppError::server_error(e))?,
        );
    }
    if let Some(jwt_lifetime) = matches.get_one::<u64>("jwt-lifetime") {
        config = config.set_jwt_lifetime(*jwt_lifetime);
    }
    if let Some(server_port) = matches.get_one::<u16>("server-port") {
        config = config.set_server_port(*server_port);
    }
    if let Some(server_workers) = matches.get_one::<usize>("server-workers") {
        config = config.set_server_workers(*server_workers);
    }
    if let Some(session_secret) = matches.get_one::<String>("session-secret") {
        config = config.set_session_secret(session_secret.to_string());
    }
    if matches.get_flag("session-secure") {
        config = config.set_session_secure(true);
    }
    if let Some(session_lifetime) = matches.get_one::<u64>("session-lifetime") {
        config = config.set_session_lifetime(*session_lifetime);
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

fn cli() -> Command {
    Command::new("daedalus")
        .about(ABOUT)
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .help("Name of the application"),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Application is in debug mode")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("db-url")
                .short('u')
                .long("db-url")
                .help("Database URL"),
        )
        .arg(
            Arg::new("db-max-conns")
                .long("db-max-conns")
                .help("Max database connections")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            Arg::new("db-conn-timeout")
                .long("db-conn-timeout")
                .help("Database connection timeout")
                .value_parser(value_parser!(u64)),
        )
        .arg(
            Arg::new("db-idle-timeout")
                .long("db-idle-timeout")
                .help("Database idle timeout")
                .value_parser(value_parser!(u64)),
        )
        .arg(
            Arg::new("db-thread-size")
                .long("db-thread-size")
                .help("Database thread size")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            Arg::new("log-level")
                .short('l')
                .long("log-level")
                .help("Log level")
                .value_parser(value_parser!(LogLevel)),
        )
        .arg(
            Arg::new("log-format")
                .short('f')
                .long("log-format")
                .help("Log format")
                .value_parser(value_parser!(LogFormat)),
        )
        .arg(
            Arg::new("jwt-pub-key")
                .long("jwt-pub-key")
                .help("JWT public key file"),
        )
        .arg(
            Arg::new("jwt-priv-key")
                .long("jwt-priv-key")
                .help("JWT private key file"),
        )
        .arg(
            Arg::new("jwt-lifetime")
                .long("jwt-lifetime")
                .help("JWT lifetime")
                .value_parser(value_parser!(u64)),
        )
        .arg(
            Arg::new("server-port")
                .short('p')
                .long("server-port")
                .help("Server port")
                .value_parser(value_parser!(u16)),
        )
        .arg(
            Arg::new("server-workers")
                .short('w')
                .long("server-workers")
                .help("Number of server workers")
                .value_parser(value_parser!(usize)),
        )
        .arg(
            Arg::new("session-secret")
                .long("session-secret")
                .help("Session secret"),
        )
        .arg(
            Arg::new("session-secure")
                .long("session-secure")
                .help("Use secure session cookies")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("session-lifetime")
                .long("session-lifetime")
                .help("Session lifetime")
                .value_parser(value_parser!(u64)),
        )
}
