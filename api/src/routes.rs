use actix_web::web::{get, post, scope, ServiceConfig};

use crate::config::AppSettings;
use crate::handlers::{tenants, users, workflows};
use crate::middleware::bearer::JwtAuth;

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
                .route("/users/{id}", post().to(users::update))
                .route("/tenants", get().to(tenants::list))
                .route("/tenants", post().to(tenants::create))
                .route("/tenants/{id}", get().to(tenants::find))
                .route("/tenants/{id}", post().to(tenants::update))
                .route("/workflows", get().to(workflows::list))
                .route("/workflows", post().to(workflows::create))
                .route("/workflows/{id}", get().to(workflows::find))
                .route("/workflows/{id}", post().to(workflows::update)),
        );
    }
}
