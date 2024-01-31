use actix_session::{
    config::PersistentSession, storage::CookieSessionStore, Session, SessionMiddleware,
};
use actix_web::{
    cookie::{self, Key},
    web::{scope, Data},
    App, HttpResponse, HttpServer,
};
use dotenvy::dotenv;
use handlebars::{DirectorySourceOptions, Handlebars};

use daedalus::{
    database::{DatabaseSettings, PoolManager},
    handlers::{tenants, users, workflows},
    services::{tenants::TenantService, users::UserService, workflows::WorkflowService},
};

async fn index(hb: Data<Handlebars<'_>>) -> HttpResponse {
    let body = hb.render("index", &()).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mut handlebars = Handlebars::new();
    if cfg!(debug_assertions) {
        handlebars.set_dev_mode(true);
    }
    handlebars
        .register_templates_directory("./views", DirectorySourceOptions::default())
        .unwrap();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PoolManager::new(&DatabaseSettings::new(database_url.clone()));
    let user_service = UserService::new(pool.get_pool());
    let workflow_service = WorkflowService::new(pool.get_pool());
    let tenant_service = TenantService::new(pool.get_pool());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(user_service.clone()))
            .app_data(Data::new(tenant_service.clone()))
            .app_data(Data::new(workflow_service.clone()))
            .app_data(Data::new(handlebars.clone()))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(cookie::time::Duration::hours(2)),
                    )
                    .build(),
            )
            .service(scope("/api/users").configure(users::configure))
            .service(scope("/api/tenants").configure(tenants::configure))
            .service(scope("/api/workflows").configure(workflows::configure))
            .route("/", actix_web::web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
