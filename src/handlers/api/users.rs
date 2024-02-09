use actix_identity::Identity;
use actix_web::{
    web::{block, Data, Json, Path, Query},
    HttpMessage, HttpRequest, Result,
};

use crate::result::AppError;
use crate::services::users::{UserCredentials, UserService};
use crate::{database::PoolManager, models::users::UpdateUser};
use crate::{
    models::users::{CreateUser, User, UserQuery},
    UserId,
};

pub async fn list(
    _: UserId,
    Query(filter): Query<UserQuery>,
    pool: Data<PoolManager>,
) -> Result<Json<Vec<User>>> {
    let users = block(move || {
        let conn = pool.get()?;
        UserService::new(conn).list(filter)
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
        UserService::new(conn).create(request)
    })
    .await??;

    Ok(Json(new_user))
}

pub async fn update(
    Json(request): Json<UpdateUser>,
    id: Path<i64>,
    pool: Data<PoolManager>,
) -> Result<Json<User>> {
    let id = id.into_inner();
    let updated_user = block(move || {
        let conn = pool.get()?;
        UserService::new(conn).update(id, request)
    })
    .await??;

    Ok(Json(updated_user))
}

pub async fn authenticate(
    Json(request): Json<UserCredentials>,
    pool: Data<PoolManager>,
    req: HttpRequest,
) -> Result<Json<User>> {
    let user = block(move || {
        let conn = pool.get()?;
        UserService::new(conn).authenticate(request)
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

pub async fn me(UserId(id): UserId, pool: Data<PoolManager>) -> Result<Json<User>> {
    let user = block(move || {
        let conn = pool.get()?;
        UserService::new(conn)
            .find(id)?
            .ok_or(AppError::Unauthorized)
    })
    .await??;

    Ok(Json(user))
}

pub async fn find(_: UserId, id: Path<i64>, pool: Data<PoolManager>) -> Result<Json<User>> {
    let id = id.into_inner();
    let user = block(move || {
        let conn = pool.get()?;
        UserService::new(conn).find(id)?.ok_or(AppError::NotFound {
            entity: "User".to_string(),
            id: id.to_string(),
        })
    })
    .await??;

    Ok(Json(user))
}
