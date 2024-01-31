use actix_web::{
    web::{scope, Data},
    App, HttpServer,
};
use dotenvy::dotenv;

use daedalus::{
    database::{DatabaseSettings, PoolManager},
    handlers::{tenants, users, workflows},
    services::{tenants::TenantService, users::UserService, workflows::WorkflowService},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

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
            .service(scope("/users").configure(users::configure))
            .service(scope("/tenants").configure(tenants::configure))
            .service(scope("/workflows").configure(workflows::configure))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
