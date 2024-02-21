use actix_identity::Identity;
use actix_web::{
    web::{block, Data, Form},
    HttpMessage, HttpRequest, HttpResponse,
};
use serde::Deserialize;

use crate::{
    database::PoolManager,
    middleware::flash_messages::{Flash, IncomingFlashMessages},
    models::users::User,
    result::{AppError, HtmlResult},
    services::users::{UserCredentials, UserService},
    tmpl::{Context, Tmpl},
    UserId,
};

#[derive(Clone, Debug, Deserialize)]
pub struct LoginFormData {
    pub email: String,
    pub password: String,
}

pub async fn show_login(
    id: Option<UserId>,
    tmpl: Data<Tmpl>,
    inbox: IncomingFlashMessages,
) -> HtmlResult<HttpResponse> {
    let _ = tmpl.reload();

    if id.is_some() {
        return Ok(HttpResponse::Found()
            .append_header(("location", "/home"))
            .finish());
    }

    let mut context = Context::new();
    context.insert("title", "Login");
    context.insert("messages", inbox.messages());
    context.insert("errors", inbox.errors());
    context.insert("data", inbox.data());
    let body = tmpl.render("pages/login.njk", &context)?;

    Ok(HttpResponse::Ok().body(body))
}

pub async fn login(
    form: Form<LoginFormData>,
    req: HttpRequest,
    pool: Data<PoolManager>,
) -> HtmlResult<HttpResponse> {
    let user = match authenticate_user(form.clone(), pool).await {
        Ok(user) => user,
        Err(e) => match e.0.clone() {
            AppError::ServerError { cause } => {
                tracing::error!("Failed to authenticate user: {}", cause);
                return Err(e);
            }
            _ => {
                Flash::error("email".into(), "Invalid email or password".into())?;
                Flash::data("email".into(), form.email.to_owned())?;
                return Ok(HttpResponse::Found()
                    .append_header(("location", "/login"))
                    .finish());
            }
        },
    };

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
