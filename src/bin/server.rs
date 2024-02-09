use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    middleware::{Compress, NormalizePath},
    web::Data,
    App, HttpServer,
};
use dotenvy::dotenv;
use tracing::Level;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{self, filter::Targets, layer::SubscriberExt, util::SubscriberInitExt};

use daedalus::{
    config::{AppSettings, ServerSettings},
    database::{Migrator, PoolManager},
    routes,
    views::View,
};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            Targets::new()
                .with_target("daedalus", Level::TRACE)
                .with_default(Level::DEBUG),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let app_settings = AppSettings::new()?;
    let server = Server::new(app_settings)?;

    server.serve().await
}

struct Server {
    settings: AppSettings,
    pool_manager: PoolManager,
}

impl Server {
    pub fn new(settings: AppSettings) -> Result<Self, Box<dyn std::error::Error>> {
        let pool_manager = PoolManager::new(&settings.database);
        Migrator::new(pool_manager.get()?)
            .run()
            .expect("Failed to run migrations");

        Ok(Self {
            settings,
            pool_manager,
        })
    }

    pub async fn serve(&self) -> Result<(), Box<dyn std::error::Error>> {
        let pool_manager = self.pool_manager.clone();
        let session_settings = self.settings.session.clone();
        let ServerSettings { port, workers } = self.settings.server.clone();

        HttpServer::new(move || {
            let session_mw = SessionMiddleware::builder(
                CookieSessionStore::default(),
                Key::from(&session_settings.secret.as_bytes()),
            )
            .cookie_secure(session_settings.secure)
            .session_lifecycle(
                PersistentSession::default()
                    .session_ttl(Duration::seconds(session_settings.lifetime.as_secs() as i64)),
            )
            .build();

            App::new()
                .app_data(Data::new(pool_manager.clone()))
                .app_data(Data::new(View::init().unwrap()))
                .wrap(IdentityMiddleware::default())
                .wrap(session_mw)
                .wrap(NormalizePath::trim())
                .wrap(Compress::default())
                .wrap(TracingLogger::default())
                .configure(routes::api)
                .configure(routes::web)
        })
        .bind(("0.0.0.0", port))?
        .workers(workers)
        .run()
        .await?;

        Ok(())
    }
}
