use actix_identity::Identity;
use actix_web::{
    web::{block, Data, Form},
    HttpMessage, HttpRequest, HttpResponse, Result,
};
use serde::Deserialize;

use crate::{
    database::PoolManager,
    result::AppError,
    services::users::{UserCredentials, UserService},
    views::{Context, View},
};

#[derive(Debug, Deserialize)]
pub struct LoginFormData {
    pub email: String,
    pub password: String,
}

pub async fn show_login(id: Option<Identity>) -> Result<HttpResponse> {
    if id.is_some() {
        return Ok(HttpResponse::Found()
            .append_header(("location", "/home"))
            .finish());
    }

    match View::render("pages/login.njk", &Context::new()) {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn login(
    form: Form<LoginFormData>,
    req: HttpRequest,
    pool: Data<PoolManager>,
) -> Result<HttpResponse> {
    let user = block(move || {
        let conn = pool.get()?;
        UserService::new(conn)
            .authenticate(UserCredentials {
                tenant_id: 1,
                email: form.email.clone(),
                password: form.password.clone(),
            })
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??;

    Identity::login(&req.extensions(), user.id.to_string()).map_err(|e| AppError::ServerError {
        cause: format!("Failed to set identity: {}", e),
    })?;

    Ok(HttpResponse::Found()
        .append_header(("location", "/home"))
        .finish())
}

pub async fn logout(id: Identity) -> Result<HttpResponse> {
    id.logout();
    Ok(HttpResponse::Found()
        .append_header(("location", "/"))
        .finish())
}
