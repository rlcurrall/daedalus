use actix_web::web::{block, Data, Json, Path, Query};
use actix_web::{get, post};

use crate::models::users::{CreateUser, User, UserQuery};
use crate::result::AppError;
use crate::services::users::{UserCredentials, UserService};

#[get("/users")]
pub async fn list_users(
    Query(filter): Query<UserQuery>,
    user_service: Data<UserService>,
) -> actix_web::Result<Json<Vec<User>>> {
    println!("filtering");
    let users = block(move || {
        user_service
            .list(filter.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??;

    Ok(Json(users))
}

#[post("/users")]
pub async fn create_user(
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

#[post("/users/authenticate")]
pub async fn authenticate_user(
    Json(request): Json<UserCredentials>,
    user_service: Data<UserService>,
) -> actix_web::Result<Json<User>> {
    let user = block(move || {
        user_service
            .authenticate(request.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??;

    Ok(Json(user))
}

#[get("/users/{id}")]
pub async fn get_user(
    id: Path<i64>,
    user_service: Data<UserService>,
) -> actix_web::Result<Json<User>> {
    let id = id.into_inner();
    let user = block(move || user_service.find(id).map_err(|e| Into::<AppError>::into(e)))
        .await??
        .ok_or(AppError::NotFound {
            entity: "User".to_string(),
            id: id.to_string(),
        })?;

    Ok(Json(user))
}
