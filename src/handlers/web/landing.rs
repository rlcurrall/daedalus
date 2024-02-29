use actix_web::web::{block, Data};
use actix_web::HttpResponse;

use crate::database::PoolManager;
use crate::models::users::User;
use crate::result::{AppError, HtmlResult};
use crate::services::users::UserService;
use crate::tmpl::{Context, Tmpl};
use crate::UserId;

pub async fn index(
    id: Option<UserId>,
    pool: Data<PoolManager>,
    tmpl: Data<Tmpl>,
) -> HtmlResult<HttpResponse> {
    let mut context = Context::new();
    context.insert("title", "Daedalus");

    if let Some(UserId(id)) = id {
        get_user(id, pool)
            .await
            .ok()
            .map(|user| context.insert("user", &user));
    };

    let body = tmpl.render("pages/index.njk", &context)?;

    Ok(HttpResponse::Ok().body(body))
}

async fn get_user(id: i64, pool: Data<PoolManager>) -> crate::result::Result<User> {
    block(move || {
        let conn = pool.get()?;
        UserService::new(conn).find(id)
    })
    .await??
    .ok_or(AppError::Unauthorized)
}
