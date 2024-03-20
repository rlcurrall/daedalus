use clap::Parser;
use console::style;
use tracing_subscriber::{self, filter::Targets, layer::SubscriberExt, util::SubscriberInitExt};

use daedalus::config::{AppSettings, ConfigBuilder, LogFormat, LogLevel};
use daedalus::result::Result;
use daedalus::server;

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

    let serve_cmd = Serve::parse();
    ConfigBuilder::new(version.into(), serve_cmd.config)
        .set_debug(serve_cmd.debug)
        .set_db_url(serve_cmd.database_url)
        .set_db_max_conns(serve_cmd.database_max_conns)
        .set_db_conn_timeout(serve_cmd.database_conn_timeout)
        .set_db_idle_timeout(serve_cmd.database_idle_timeout)
        .set_db_thread_size(serve_cmd.database_thread_size)
        .set_log_level(serve_cmd.log_level)
        .set_log_format(serve_cmd.log_format)
        .set_jwt_pub_key(serve_cmd.jwt_pub_key)
        .set_jwt_priv_key(serve_cmd.jwt_priv_key)
        .set_jwt_lifetime(serve_cmd.jwt_lifetime)
        .set_server_port(serve_cmd.server_port)
        .set_server_workers(serve_cmd.server_workers)
        .parse()
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
    #[clap(short, long)]
    pub config: Option<String>,
    #[clap(short = 'd', long)]
    pub debug: Option<bool>,
    #[clap(short = 'D', long)]
    pub database_url: Option<String>,
    #[clap(long)]
    pub database_max_conns: Option<u32>,
    #[clap(long)]
    pub database_conn_timeout: Option<u64>,
    #[clap(long)]
    pub database_idle_timeout: Option<u64>,
    #[clap(long)]
    pub database_thread_size: Option<u32>,
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
}
