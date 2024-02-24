use actix_identity::IdentityMiddleware;
use actix_web::web::{get, post, resource, scope, Path, ServiceConfig};
use actix_web::HttpResponse;
use rust_embed::RustEmbed;

use crate::config::AppSettings;
use crate::handlers::api::{tenants, users, workflows};
use crate::handlers::web::{auth, home, landing};
use crate::middleware::bearer::JwtAuth;
use crate::middleware::flash::{FlashMiddleware, SessionStore};
use crate::middleware::session::SessionMiddlewareBuilder;

pub fn api_routes(
    settings: AppSettings,
) -> impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static {
    move |cfg: &mut ServiceConfig| {
        cfg.service(
            scope("/api")
                .wrap(JwtAuth::new(settings.jwt.pub_key.clone()))
                .service(
                    scope("/users")
                        .route("/me", get().to(users::me))
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
}

pub fn web_routes(
    settings: AppSettings,
) -> impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static {
    move |cfg: &mut ServiceConfig| {
        cfg.service(
            scope("")
                .wrap(SessionMiddlewareBuilder::build(&settings.session))
                .wrap(IdentityMiddleware::default())
                .wrap(FlashMiddleware::new(SessionStore::default()))
                .service(resource("/").name("landing_page").get(landing::index))
                .service(resource("/home").name("home").get(home::index))
                .service(
                    resource("/logout")
                        .name("logout")
                        .get(auth::logout)
                        .post(auth::logout),
                )
                .service(
                    resource("/login")
                        .name("login")
                        .get(auth::show_login)
                        .post(auth::login),
                )
                .route("/{version}/{path:.*}", get().to(static_files)),
        );
    }
}

#[derive(RustEmbed)]
#[folder = "resource/assets"]
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
        Err(e) => {
            tracing::info!("Error reading file: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
