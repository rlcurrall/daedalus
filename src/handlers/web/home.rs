use actix_identity::Identity;
use actix_web::{
    web::{block, Data},
    HttpResponse, Result,
};

use crate::{
    database::PoolManager,
    result::AppError,
    services::users::UserService,
    views::{Context, View},
};

pub async fn index(id: Identity, pool: Data<PoolManager>) -> Result<HttpResponse> {
    View::reload()?;

    let user_id = id
        .id()
        .map_err(|_| AppError::Unauthorized)?
        .parse::<i64>()
        .map_err(|_| AppError::Unauthorized)?;

    let user = block(move || {
        let conn = pool.get()?;
        UserService::new(conn)
            .find(user_id)?
            .ok_or(AppError::Unauthorized)
    })
    .await??;

    let mut context = Context::new();
    context.insert("user", &user);

    let body = View::render("pages/home.njk", &context)?;

    Ok(HttpResponse::Ok().body(body))
}
