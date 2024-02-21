use actix_identity::IdentityMiddleware;
use actix_web::{
    middleware::{Compress, NormalizePath},
    web::{get, post, resource, scope, Data, Path},
    App, HttpResponse, HttpServer,
};
use rust_embed::RustEmbed;
use tracing_actix_web::TracingLogger;

use crate::{
    config::{AppSettings, ServerSettings},
    database::PoolManager,
    handlers::{api, web},
    middleware::{
        flash_messages::{FlashMiddlewareBuilder, SessionMessageStore},
        session::SessionMiddlewareBuilder,
    },
    tmpl::Tmpl,
};

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
            .wrap(IdentityMiddleware::default())
            .wrap(FlashMiddlewareBuilder::new(SessionMessageStore::default()))
            .wrap(SessionMiddlewareBuilder::build(&settings.session))
            .wrap(NormalizePath::trim())
            .wrap(Compress::default())
            .wrap(TracingLogger::default())
            .configure(api_routes)
            .configure(web_routes)
    })
    .bind(("0.0.0.0", port))?
    .workers(workers)
    .run()
    .await?;

    Ok(())
}

fn api_routes(cfg: &mut actix_web::web::ServiceConfig) {
    use api::{tenants, users, workflows};

    cfg.service(
        scope("/api")
            .service(
                scope("/users")
                    .route("/me", get().to(users::me))
                    .route("/logout", post().to(users::logout))
                    .route("/authenticate", post().to(users::authenticate)),
            )
            .service(
                resource("/users")
                    .name("user_collection")
                    .get(users::list)
                    .post(users::create),
            )
            .service(
                resource("/users/{id}")
                    .name("user_detail")
                    .get(users::find)
                    .post(users::update),
            )
            .service(
                resource("/tenants")
                    .name("tenant_collection")
                    .get(tenants::list)
                    .post(tenants::create),
            )
            .service(
                resource("/tenants/{id}")
                    .name("tenant_detail")
                    .get(tenants::find)
                    .post(tenants::update),
            )
            .service(
                resource("/workflows")
                    .name("workflow_collection")
                    .get(workflows::list)
                    .post(workflows::create),
            )
            .service(
                resource("/workflows/{id}")
                    .name("workflow_detail")
                    .get(workflows::find)
                    .post(workflows::update),
            ),
    );
}

fn web_routes(cfg: &mut actix_web::web::ServiceConfig) {
    use web::{auth, home, landing};

    cfg.service(resource("/").name("landing_page").get(landing::index))
        .service(resource("/home").name("home").get(home::index))
        .service(
            resource("/login")
                .name("login")
                .get(auth::show_login)
                .post(auth::login),
        )
        .service(resource("/logout").get(auth::logout))
        .route("/{version}/{path:.*}", get().to(static_files));
}

#[derive(RustEmbed)]
#[folder = "static"]
struct StaticAssets;

pub async fn static_files(path: Path<(String, String)>) -> HttpResponse {
    let path = path.1.to_owned();
    let file = match StaticAssets::get(&path) {
        Some(file) => file,
        None => return HttpResponse::NotFound().finish(),
    };

    let mimetype = file.metadata.mimetype();

    match String::from_utf8(file.data.into_owned()) {
        Ok(content) => HttpResponse::Ok().content_type(mimetype).body(content),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
