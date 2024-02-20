use actix_identity::Identity;
use actix_web::{
    web::{block, Data, Form},
    HttpMessage, HttpRequest, HttpResponse,
};
use serde::Deserialize;

use crate::{
    database::PoolManager,
    models::users::User,
    result::HtmlResult,
    services::users::{UserCredentials, UserService},
    tmpl::{Context, Tmpl},
    UserId,
};

#[derive(Debug, Deserialize)]
pub struct LoginFormData {
    pub email: String,
    pub password: String,
}

pub async fn show_login(id: Option<UserId>, tmpl: Data<Tmpl>) -> HtmlResult<HttpResponse> {
    let _ = tmpl.reload();

    if id.is_some() {
        return Ok(HttpResponse::Found()
            .append_header(("location", "/home"))
            .finish());
    }

    let mut context = Context::new();
    context.insert("title", "Login");
    let body = tmpl.render("pages/login.njk", &context)?;

    Ok(HttpResponse::Ok().body(body))
}

pub async fn login(
    form: Form<LoginFormData>,
    req: HttpRequest,
    pool: Data<PoolManager>,
) -> HtmlResult<HttpResponse> {
    let user = authenticate_user(form.into_inner(), pool).await?;

    Identity::login(&req.extensions(), user.id.to_string())?;

    Ok(HttpResponse::Found()
        .append_header(("location", "/home"))
        .finish())
}

pub async fn logout(id: Option<Identity>) -> HtmlResult<HttpResponse> {
    if let Some(id) = id {
        id.logout()
    }

    Ok(HttpResponse::Found()
        .append_header(("location", "/"))
        .finish())
}

async fn authenticate_user(form: LoginFormData, pool: Data<PoolManager>) -> HtmlResult<User> {
    Ok(block(move || {
        let conn = pool.get()?;
        UserService::new(conn).authenticate(UserCredentials {
            tenant_id: 1,
            email: form.email.clone(),
            password: form.password.clone(),
        })
    })
    .await??)
}
