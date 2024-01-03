use actix_web::{web::Data, App, HttpServer};
use dotenvy::dotenv;

use daedalus::{
    database::{DatabaseSettings, PoolManager},
    handlers::{tenants, users},
    services::{tenants::TenantService, users::UserService},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    println!("Starting server...");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PoolManager::new(&DatabaseSettings::new(database_url.clone()));
    let user_service = UserService::new(pool.get_pool());
    let tenant_service = TenantService::new(pool.get_pool());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(user_service.clone()))
            .app_data(Data::new(tenant_service.clone()))
            .service(users::list_users)
            .service(users::create_user)
            .service(users::get_user)
            .service(users::authenticate_user)
            .service(tenants::list_tenants)
            .service(tenants::create_tenant)
            .service(tenants::get_tenant)
            .service(tenants::update_tenant)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
