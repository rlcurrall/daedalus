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

pub async fn index(id: Option<Identity>, pool: Data<PoolManager>) -> Result<HttpResponse> {
    View::reload()?;

    let user = match id
        .map(|id| id.id().ok().and_then(|id| id.parse::<i64>().ok()))
        .flatten()
    {
        None => None,
        Some(id) => Some(
            block(move || {
                let conn = pool.get()?;
                UserService::new(conn)
                    .find(id)
                    .map_err(|e| Into::<AppError>::into(e))
            })
            .await??,
        ),
    };

    let mut context = Context::new();
    if let Some(user) = user {
        context.insert("user", &user);
    };

    let body = View::render("pages/index.njk", &context)?;

    Ok(HttpResponse::Ok().body(body))
}
