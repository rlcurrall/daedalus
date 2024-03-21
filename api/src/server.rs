use actix_web::middleware::{Compress, NormalizePath};
use actix_web::web::{Data, JsonConfig};
use actix_web::{App, HttpServer};
use tracing_actix_web::TracingLogger;

use crate::api::api_routes;
use crate::config::{AppSettings, ServerSettings};
use crate::database::PoolManager;

pub async fn start(settings: AppSettings) -> Result<(), Box<dyn std::error::Error>> {
    let mut pool_manager = PoolManager::new(&settings.database);
    let ServerSettings { port, workers } = settings.server.clone();

    pool_manager.migrate()?;

    tracing::info!("Starting server at: http://localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(JsonConfig::default().error_handler(|err, _req| {
                let json = serde_json::json!({
                    "error": "Failed to parse JSON",
                    "detail": err.to_string()
                });
                actix_web::error::InternalError::from_response(
                    err,
                    actix_web::HttpResponse::UnprocessableEntity().json(json),
                )
                .into()
            }))
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
