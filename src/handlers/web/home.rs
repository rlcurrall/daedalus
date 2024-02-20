use actix_web::{
    web::{block, Data},
    HttpResponse,
};

use crate::{
    database::PoolManager,
    result::{AppError, HtmlResult},
    services::users::UserService,
    tmpl::{Context, Tmpl},
    UserId,
};

pub async fn index(
    id: Option<UserId>,
    pool: Data<PoolManager>,
    tmpl: Data<Tmpl>,
) -> HtmlResult<HttpResponse> {
    let _ = tmpl.reload();

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

    let body = tmpl.render("home/page.njk", &context)?;

    Ok(HttpResponse::Ok().body(body))
}
