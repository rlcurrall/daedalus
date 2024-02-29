use actix_web::web::{block, Data};
use actix_web::HttpResponse;

use crate::database::PoolManager;
use crate::result::{AppError, HtmlResult};
use crate::services::users::UserService;
use crate::tmpl::{Context, Tmpl};
use crate::UserId;

pub async fn index(
    id: Option<UserId>,
    pool: Data<PoolManager>,
    tmpl: Data<Tmpl>,
) -> HtmlResult<HttpResponse> {
    let UserId(id) = id.ok_or(AppError::Unauthorized)?;
    let user = block(move || {
        let conn = pool.get()?;
        UserService::new(conn).find(id)
    })
    .await??
    .ok_or(AppError::Unauthorized)?;

    let mut context = Context::new();
    context.insert("title", "Home");
    context.insert("user", &user);

    let body = tmpl.render("pages/home.njk", &context)?;

    Ok(HttpResponse::Ok().body(body))
}
