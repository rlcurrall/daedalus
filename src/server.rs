use actix_web::middleware::{Compress, NormalizePath};
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use tracing_actix_web::TracingLogger;

use crate::config::{AppSettings, ServerSettings};
use crate::database::PoolManager;
use crate::routes::{api_routes, web_routes};
use crate::tmpl::Tmpl;

pub async fn start(settings: AppSettings) -> Result<(), Box<dyn std::error::Error>> {
    let mut pool_manager = PoolManager::new(&settings.database);
    let templates = Tmpl::init(settings.version.clone())?;
    let ServerSettings { port, workers } = settings.server.clone();

    pool_manager.migrate()?;

    tracing::info!("Starting server on port: {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(settings.clone()))
            .app_data(Data::new(pool_manager.clone()))
            .app_data(Data::new(templates.clone()))
            .wrap(NormalizePath::trim())
            .wrap(Compress::default())
            .wrap(TracingLogger::default())
            .configure(api_routes(settings.clone()))
            .configure(web_routes(settings.clone()))
    })
    .bind(("0.0.0.0", port))?
    .workers(workers)
    .run()
    .await?;

    Ok(())
}
