use actix_web::web::{get, post, resource, scope, ServiceConfig};

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
