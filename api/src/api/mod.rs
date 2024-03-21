use actix_web::web::{get, patch, post, scope, ServiceConfig};
use serde::{Deserialize, Serialize};
use tsync::tsync;

use crate::{config::AppSettings, middleware::bearer::JwtAuth};

pub mod tenants;
pub mod users;
pub mod workflows;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[tsync]
pub struct Paginated<T> {
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub data: Vec<T>,
}

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
                .route("/users/{id}", patch().to(users::update))
                .route("/tenants", get().to(tenants::list))
                .route("/tenants", post().to(tenants::create))
                .route("/tenants/{id}", get().to(tenants::find))
                .route("/tenants/{id}", patch().to(tenants::update))
                .route("/workflows", get().to(workflows::list))
                .route("/workflows", post().to(workflows::create))
                .route("/workflows/{id}", get().to(workflows::find))
                .route("/workflows/{id}", patch().to(workflows::update)),
        );
    }
}
