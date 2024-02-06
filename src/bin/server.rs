use std::{fs::File, time::SystemTime};

use actix_identity::IdentityMiddleware;
use actix_web::{
    middleware::{Compress, NormalizePath},
    web::Data,
    App, HttpServer,
};
use dotenvy::dotenv;
use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};
use tracing_actix_web::TracingLogger;

use daedalus::{
    config::{AppSettings, ServerSettings},
    database::{Migrator, PoolManager},
    middleware::session::SessionMiddlewareBuilder,
    routes,
    views::View,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let app_settings = AppSettings::new().expect("Failed to load settings");

    setup_logger(&app_settings.name);
    View::init()?;

    let pool_manager = PoolManager::new(&app_settings.database);
    Migrator::new(pool_manager.get()?)
        .run()
        .expect("Failed to run migrations");

    let ServerSettings { port, workers } = app_settings.server.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool_manager.clone()))
            .wrap(NormalizePath::trim())
            .wrap(IdentityMiddleware::default())
            .wrap(Compress::default())
            .wrap(SessionMiddlewareBuilder::build(
                app_settings.session.clone(),
            ))
            .wrap(TracingLogger::default())
            .configure(routes::api)
            .configure(routes::web)
    })
    .bind(("0.0.0.0", port))?
    .workers(workers)
    .run()
    .await
}

fn setup_logger(app_name: &str) {
    let filename = format!(
        "logs/{}-{}.log",
        app_name,
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(filename).unwrap(),
        ),
    ])
    .unwrap();
}
