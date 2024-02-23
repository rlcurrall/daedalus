use std::fs::read_to_string;
use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::web::{block, Data, Json, Path, Query};
use actix_web::HttpRequest;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

use crate::config::AppSettings;
use crate::middleware::bearer::UserClaims;
use crate::models::users::{CreateUser, User, UserQuery};
use crate::result::{AppError, JsonResult};
use crate::services::users::{UserCredentials, UserService};
use crate::{database::PoolManager, models::users::UpdateUser};

pub async fn list(
    _: UserClaims,
    Query(filter): Query<UserQuery>,
    pool: Data<PoolManager>,
) -> JsonResult<Json<Vec<User>>> {
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
) -> JsonResult<Json<User>> {
    let new_user = block(move || {
        let conn = pool.get()?;
        UserService::new(conn).create(request)
    })
    .await??;

    Ok(Json(new_user))
}

pub async fn update(
    _: UserClaims,
    Json(request): Json<UpdateUser>,
    id: Path<i64>,
    pool: Data<PoolManager>,
) -> JsonResult<Json<User>> {
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
    setting: Data<AppSettings>,
    _req: HttpRequest,
) -> JsonResult<Json<serde_json::Value>> {
    let user = block(move || {
        let conn = pool.get()?;
        UserService::new(conn).authenticate(request)
    })
    .await??;

    let priv_key = read_to_string(setting.jwt.priv_key.clone())?;
    let key = EncodingKey::from_rsa_pem(priv_key.as_bytes())?;
    let header = Header::new(jsonwebtoken::Algorithm::RS256);
    let exp = SystemTime::now()
        .checked_add(setting.jwt.lifetime.clone())
        .ok_or(AppError::server_error("Failed to set token expiration"))?
        .duration_since(UNIX_EPOCH)?
        .as_secs() as usize;
    let claims = UserClaims::new(user.id, exp, vec![]);
    let token = encode(&header, &claims, &key)?;

    Ok(Json(json!({
        "token": token,
        "user": user,
    })))
}

pub async fn me(claims: UserClaims, pool: Data<PoolManager>) -> JsonResult<Json<User>> {
    let user = block(move || {
        let conn = pool.get()?;
        UserService::new(conn)
            .find(claims.sub)?
            .ok_or(AppError::Unauthorized)
    })
    .await??;

    Ok(Json(user))
}

pub async fn find(_: UserClaims, id: Path<i64>, pool: Data<PoolManager>) -> JsonResult<Json<User>> {
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
