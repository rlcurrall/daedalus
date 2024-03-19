use actix_web::web::{get, post, put, scope, ServiceConfig};

use crate::{config::AppSettings, middleware::bearer::JwtAuth};

pub mod tenants;
pub mod users;
pub mod workflows;

pub fn api_routes(
    settings: AppSettings,
) -> impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static {
    move |cfg: &mut ServiceConfig| {
        cfg.service(
            scope("/api")
                .wrap(JwtAuth::new(settings.jwt.pub_key.clone()))
                .route("/users", get().to(users::list))
                .route("/users", post().to(users::create))
                .route("/users/me", get().to(users::me))
                .route("/users/authenticate", post().to(users::authenticate))
                .route("/users/{id}", get().to(users::find))
                .route("/users/{id}", put().to(users::update))
                .route("/tenants", get().to(tenants::list))
                .route("/tenants", post().to(tenants::create))
                .route("/tenants/{id}", get().to(tenants::find))
                .route("/tenants/{id}", put().to(tenants::update))
                .route("/workflows", get().to(workflows::list))
                .route("/workflows", post().to(workflows::create))
                .route("/workflows/{id}", get().to(workflows::find))
                .route("/workflows/{id}", put().to(workflows::update)),
        );
    }
}
