use actix_web::{
    dev::ServiceResponse,
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    web::{get, post, resource, scope},
    HttpResponse, ResponseError, Result,
};

use crate::handlers::{api, web};

pub fn api(cfg: &mut actix_web::web::ServiceConfig) {
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

pub fn web(cfg: &mut actix_web::web::ServiceConfig) {
    use web::{auth, home, landing};

    cfg.service(resource("/").name("landing_page").get(landing::index))
        .service(resource("/home").name("home").get(home::index))
        .service(
            resource("/login")
                .name("login")
                .get(auth::show_login)
                .post(auth::login),
        )
        .service(resource("/logout").get(auth::logout));
}
