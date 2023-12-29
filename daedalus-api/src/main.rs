use actix_web::{web::Data, App, HttpServer};
use daedalus_core::{tenants::TenantService, users::UserService, DatabaseSettings, PoolManager};
use dotenvy::dotenv;
use paperclip::actix::OpenApiExt;

mod http;
mod result;
mod tenants;
mod users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PoolManager::new(&DatabaseSettings::new(database_url.clone()));
    let user_service = UserService::new(pool.get_pool());
    let tenant_service = TenantService::new(pool.get_pool());

    HttpServer::new(move || {
        App::new()
            .wrap_api()
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
            .with_json_spec_at("/swagger.v2.json")
            .with_swagger_ui_at("/docs")
            .build()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
