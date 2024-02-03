use actix_identity::Identity;
use actix_web::{
    web::{block, Data, Json, Path, Query},
    HttpMessage, HttpRequest, Result,
};

use crate::database::PoolManager;
use crate::models::users::{CreateUser, User, UserQuery};
use crate::result::AppError;
use crate::services::users::{UserCredentials, UserService};

pub async fn list(
    _: Identity,
    Query(filter): Query<UserQuery>,
    pool: Data<PoolManager>,
) -> Result<Json<Vec<User>>> {
    let users = block(move || {
        let conn = pool.get()?;
        UserService::new(conn)
            .list(filter.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??;

    Ok(Json(users))
}

pub async fn create(
    Json(request): Json<CreateUser>,
    pool: Data<PoolManager>,
) -> Result<Json<User>> {
    let new_user = block(move || {
        let conn = pool.get()?;
        UserService::new(conn)
            .create(request.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??;

    Ok(Json(new_user))
}

pub async fn authenticate(
    Json(request): Json<UserCredentials>,
    pool: Data<PoolManager>,
    req: HttpRequest,
) -> Result<Json<User>> {
    let user = block(move || {
        let conn = pool.get()?;
        UserService::new(conn)
            .authenticate(request.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??;

    Identity::login(&req.extensions(), user.id.to_string()).map_err(|e| AppError::ServerError {
        cause: format!("Failed to set identity: {}", e),
    })?;

    Ok(Json(user))
}

pub async fn logout(id: Identity) -> Result<Json<()>> {
    id.logout();
    Ok(Json(()))
}

pub async fn me(id: Identity, pool: Data<PoolManager>) -> Result<Json<User>> {
    let id = id.id().map_err(|_| AppError::Unauthorized)?;

    let user = block(move || {
        let conn = pool.get()?;
        UserService::new(conn)
            .find(id.parse().map_err(|_| AppError::Unauthorized)?)
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??
    .ok_or(AppError::Unauthorized)?;

    Ok(Json(user))
}

pub async fn find(_: Identity, id: Path<i64>, pool: Data<PoolManager>) -> Result<Json<User>> {
    let id = id.into_inner();
    let user = block(move || {
        let conn = pool.get()?;
        UserService::new(conn)
            .find(id)
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??
    .ok_or(AppError::NotFound {
        entity: "User".to_string(),
        id: id.to_string(),
    })?;

    Ok(Json(user))
}
