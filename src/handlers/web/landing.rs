use actix_web::{
    web::{block, Data},
    HttpResponse,
};
use tracing::error;

use crate::{
    database::PoolManager,
    models::users::User,
    result::AppError,
    services::users::UserService,
    views::{Context, View},
    UserId,
};

pub async fn index(id: Option<UserId>, pool: Data<PoolManager>, views: Data<View>) -> HttpResponse {
    if let Err(e) = views.reload() {
        error!("Failed to reload views: {}", e);
        return HttpResponse::InternalServerError().finish();
    }

    let mut context = Context::new();

    if let Some(UserId(id)) = id {
        let user = match get_user(id, pool).await {
            Ok(user) => user,
            Err(e) => {
                error!("Failed to get user: {}", e);
                return HttpResponse::InternalServerError().finish();
            }
        };

        context.insert("user", &user);
    };

    match views.render("pages/index.njk", &context) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(e) => {
            error!("Failed to render view: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn get_user(id: i64, pool: Data<PoolManager>) -> crate::result::Result<User> {
    block(move || {
        let conn = pool.get()?;
        UserService::new(conn).find(id)
    })
    .await
    .map_err(|_| AppError::Unauthorized)??
    .ok_or(AppError::Unauthorized)
}
