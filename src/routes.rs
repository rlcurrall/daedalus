use actix_identity::IdentityMiddleware;
use actix_web::web::{get, post, resource, scope, Data, ServiceConfig};
use actix_web::{HttpRequest, HttpResponse};
use rust_embed::RustEmbed;
use tera::Context;

use crate::config::AppSettings;
use crate::handlers::api::{tenants, users, workflows};
use crate::handlers::web::{auth, home, landing};
use crate::middleware::bearer::JwtAuth;
use crate::middleware::flash::{FlashMiddleware, SessionStore};
use crate::middleware::session::SessionMiddlewareBuilder;
use crate::tmpl::Tmpl;

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
                .service(
                    resource("/register")
                        .name("register")
                        .get(auth::show_register)
                        .post(auth::register),
                )
                .default_service(get().to(render_views)),
        );
    }
}

#[derive(RustEmbed)]
#[folder = "public"]
struct PublicFiles;

pub async fn render_views(req: HttpRequest, tmpl: Data<Tmpl>) -> HttpResponse {
    let path = req.path();
    let path = path.strip_prefix('/').unwrap_or(path);

    // Check if the path is a template that did not have an explicit route defined
    let template_name = to_template_name(&path);
    if let Ok(body) = tmpl.render(&template_name, &Context::new()) {
        return HttpResponse::Ok().content_type("text/html").body(body);
    }

    // Check if the path is a static asset
    if let Some(file) = PublicFiles::get(&path) {
        let mimetype = file.metadata.mimetype();
        return match String::from_utf8(file.data.into_owned()) {
            Ok(content) => HttpResponse::Ok().content_type(mimetype).body(content),
            Err(e) => {
                tracing::info!("Error reading file: {e}");
                HttpResponse::InternalServerError().finish()
            }
        };
    }

    HttpResponse::NotFound().finish()
}

pub const DEFAULT_TEMPLATE: &str = "pages/index.njk";
pub fn to_template_name(request_path: &str) -> String {
    if request_path.eq("") {
        DEFAULT_TEMPLATE.into()
    } else {
        format!("pages/{request_path}.njk")
    }
}
