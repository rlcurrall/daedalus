use actix_identity::Identity;
use actix_web::web::{block, Data, Json, Path, Query};
use actix_web::{HttpMessage, HttpRequest};

use crate::models::users::{CreateUser, User, UserQuery};
use crate::result::AppError;
use crate::services::users::{UserCredentials, UserService};

pub async fn list(
    Query(filter): Query<UserQuery>,
    user_service: Data<UserService>,
) -> actix_web::Result<Json<Vec<User>>> {
    let users = block(move || {
        user_service
            .list(filter.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??;

    Ok(Json(users))
}

pub async fn create(
    Json(request): Json<CreateUser>,
    user_service: Data<UserService>,
) -> actix_web::Result<Json<User>> {
    let new_user = block(move || {
        user_service
            .create(request.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??;

    Ok(Json(new_user))
}

pub async fn authenticate(
    Json(request): Json<UserCredentials>,
    user_service: Data<UserService>,
    req: HttpRequest,
) -> actix_web::Result<Json<User>> {
    let user = block(move || {
        user_service
            .authenticate(request.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??;

    Identity::login(&req.extensions(), user.id.to_string()).map_err(|e| AppError::ServerError {
        cause: format!("Failed to set identity: {}", e),
    })?;

    Ok(Json(user))
}

pub async fn logout(id: Identity) -> actix_web::Result<Json<()>> {
    id.logout();
    Ok(Json(()))
}

pub async fn me(id: Identity, user_service: Data<UserService>) -> actix_web::Result<Json<User>> {
    let id = id.id().map_err(|_| AppError::Unauthorized)?;

    let user = block(move || {
        user_service
            .find(id.parse().map_err(|_| AppError::Unauthorized)?)
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??
    .ok_or(AppError::Unauthorized)?;

    Ok(Json(user))
}

pub async fn find(id: Path<i64>, user_service: Data<UserService>) -> actix_web::Result<Json<User>> {
    let id = id.into_inner();
    let user = block(move || user_service.find(id).map_err(|e| Into::<AppError>::into(e)))
        .await??
        .ok_or(AppError::NotFound {
            entity: "User".to_string(),
            id: id.to_string(),
        })?;

    Ok(Json(user))
}
