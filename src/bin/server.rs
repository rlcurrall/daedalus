use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{self, Key},
    middleware::{Logger, NormalizePath},
    web::{scope, Data},
    App, HttpResponse, HttpServer,
};
use askama::Template;
use dotenvy::dotenv;
use tracing_actix_web::TracingLogger;

use daedalus::{
    database::{DatabaseSettings, PoolManager},
    handlers::{tenants, users, workflows},
    services::{tenants::TenantService, users::UserService, workflows::WorkflowService},
};

async fn index() -> HttpResponse {
    let body = HomePage.render().unwrap();
    HttpResponse::Ok().body(body)
}

#[derive(Clone, Debug)]
struct AppSettings {
    _debug: bool,
    database: DatabaseSettings,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // todo: source this from a file or env vars
    let app_settings = AppSettings {
        _debug: cfg!(debug_assertions),
        database: DatabaseSettings {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            max_connections: None,
            idle_timeout: None,
            connection_timeout: None,
            thread_pool_size: None,
        },
    };

    let pool_manager = PoolManager::new(&app_settings.database);
    let user_service = UserService::new(pool_manager.get_pool());
    let workflow_service = WorkflowService::new(pool_manager.get_pool());
    let tenant_service = TenantService::new(pool_manager.get_pool());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(user_service.clone()))
            .app_data(Data::new(tenant_service.clone()))
            .app_data(Data::new(workflow_service.clone()))
            .app_data(Data::new(app_settings.clone()))
            .wrap(NormalizePath::trim())
            .wrap(Logger::default())
            .wrap(TracingLogger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(cookie::time::Duration::hours(2)),
                    )
                    .build(),
            )
            .configure(configure_web)
            .configure(configure_api)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

pub fn configure_api(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        scope("/api")
            .configure(users::configure)
            .configure(tenants::configure)
            .configure(workflows::configure),
    );
}

#[derive(Template)]
#[template(path = "pages/index.j2")]
struct HomePage;

pub fn configure_web(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.route("/", actix_web::web::get().to(index));
}
