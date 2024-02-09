use actix_identity::Identity;
use actix_web::{
    web::{block, Data, Form},
    HttpMessage, HttpRequest, HttpResponse, Result,
};
use serde::Deserialize;
use tracing::error;

use crate::{
    database::PoolManager,
    models::users::User,
    result::AppError,
    services::users::{UserCredentials, UserService},
    views::{Context, View},
    UserId,
};

#[derive(Debug, Deserialize)]
pub struct LoginFormData {
    pub email: String,
    pub password: String,
}

pub async fn show_login(id: Option<UserId>, views: Data<View>) -> HttpResponse {
    if id.is_some() {
        return HttpResponse::Found()
            .append_header(("location", "/home"))
            .finish();
    }

    match views.render("pages/login.njk", &Context::new()) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(e) => {
            error!("Failed to render view: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn login(
    form: Form<LoginFormData>,
    req: HttpRequest,
    pool: Data<PoolManager>,
) -> HttpResponse {
    let user = match authenticate_user(form.into_inner(), pool).await {
        Ok(user) => user,
        Err(e) => {
            error!("Failed to authenticate user: {}", e);
            return HttpResponse::Found()
                .append_header(("location", "/login"))
                .finish();
        }
    };

    if let Err(e) =
        Identity::login(&req.extensions(), user.id.to_string()).map_err(|e| AppError::ServerError {
            cause: format!("Failed to set identity: {}", e),
        })
    {
        error!("Failed to set identity: {}", e);
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Found()
        .append_header(("location", "/home"))
        .finish()
}

pub async fn logout(id: Identity) -> Result<HttpResponse> {
    id.logout();
    Ok(HttpResponse::Found()
        .append_header(("location", "/"))
        .finish())
}

async fn authenticate_user(
    form: LoginFormData,
    pool: Data<PoolManager>,
) -> crate::result::Result<User> {
    block(move || {
        let conn = pool.get()?;
        UserService::new(conn).authenticate(UserCredentials {
            tenant_id: 1,
            email: form.email.clone(),
            password: form.password.clone(),
        })
    })
    .await
    .map_err(|_| AppError::Unauthorized)?
}
