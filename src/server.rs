use actix_web::middleware::{Compress, NormalizePath};
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use tracing_actix_web::TracingLogger;

use crate::config::{AppSettings, ServerSettings};
use crate::database::PoolManager;
use crate::routes::api_routes;

pub async fn start(settings: AppSettings) -> Result<(), Box<dyn std::error::Error>> {
    let mut pool_manager = PoolManager::new(&settings.database);
    let ServerSettings { port, workers } = settings.server.clone();

    pool_manager.migrate()?;

    tracing::info!("Starting server at: http://localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(settings.clone()))
            .app_data(Data::new(pool_manager.clone()))
            .wrap(NormalizePath::trim())
            .wrap(Compress::default())
            .wrap(TracingLogger::default())
            .configure(api_routes(settings.clone()))
    })
    .bind(("0.0.0.0", port))?
    .workers(workers)
    .run()
    .await?;

    Ok(())
}
