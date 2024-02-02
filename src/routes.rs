use actix_web::{
    web::{get, post, scope},
    HttpResponse,
};
use askama::Template;

use crate::handlers::{tenants, users, workflows};

pub fn api(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        scope("/api")
            .service(
                scope("/users")
                    .route("", get().to(users::list))
                    .route("", post().to(users::create))
                    .route("/authenticate", post().to(users::authenticate))
                    .route("/logout", post().to(users::logout))
                    .route("/me", get().to(users::me))
                    .route("/{id}", get().to(users::find)),
            )
            .service(
                actix_web::web::scope("/tenants")
                    .route("", post().to(tenants::create))
                    .route("", get().to(tenants::list))
                    .route("/{id}", get().to(tenants::find))
                    .route("/{id}", post().to(tenants::update)),
            )
            .service(
                actix_web::web::scope("/workflows")
                    .route("", get().to(workflows::list))
                    .route("", post().to(workflows::create))
                    .route("/{id}", get().to(workflows::get))
                    .route("/{id}", post().to(workflows::update)),
            ),
    );
}

pub fn web(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.route("/", get().to(index));
}

async fn index() -> HttpResponse {
    #[derive(Template)]
    #[template(path = "pages/index.j2")]
    struct HomePage;

    let body = HomePage.render().unwrap();
    HttpResponse::Ok().body(body)
}
