use std::{fs::File, time::SystemTime};

use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{self, Key},
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
    config::{AppSettings, ServerSettings, SessionSettings},
    database::{Migrator, PoolManager},
    routes,
    services::{tenants::TenantService, users::UserService, workflows::WorkflowService},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let app_settings = AppSettings::new().expect("Failed to load settings");

    setup_logger(&app_settings.name);

    let pool_manager = PoolManager::new(&app_settings.database);
    let migrator = Migrator::new(pool_manager.get_pool());
    migrator.run().expect("Failed to run migrations");

    let ServerSettings { port, workers } = app_settings.server.clone();
    HttpServer::new(move || {
        let pool_manager = pool_manager.clone();
        let user_service = UserService::new(pool_manager.get_pool());
        let workflow_service = WorkflowService::new(pool_manager.get_pool());
        let tenant_service = TenantService::new(pool_manager.get_pool());

        let SessionSettings { secret, lifetime } = app_settings.session.clone();
        let session_middleware = SessionMiddleware::builder(
            CookieSessionStore::default(),
            Key::from(&secret.as_bytes()),
        )
        .cookie_secure(false)
        .session_lifecycle(
            PersistentSession::default()
                .session_ttl(cookie::time::Duration::seconds(lifetime.as_secs() as i64)),
        )
        .build();

        App::new()
            .app_data(Data::new(user_service))
            .app_data(Data::new(tenant_service))
            .app_data(Data::new(workflow_service))
            .wrap(NormalizePath::trim())
            .wrap(IdentityMiddleware::default())
            .wrap(Compress::default())
            .wrap(session_middleware)
            .wrap(TracingLogger::default())
            .configure(routes::web)
            .configure(routes::api)
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
