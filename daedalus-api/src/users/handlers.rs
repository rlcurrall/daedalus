use actix_web::web::{self, Data, Json, Path, Query};
use daedalus_core::users::UserService;
use paperclip::actix::{api_v2_operation, get, post};

use crate::{
    result::AppError,
    users::models::{AuthenticateRequest, CreateUserRequest, UserListRequest, UserResponse},
};

#[api_v2_operation(tags(Users))]
#[get("/users")]
pub async fn list_users(
    Query(filter): Query<UserListRequest>,
    user_service: Data<UserService>,
) -> actix_web::Result<Json<Vec<UserResponse>>> {
    let users = web::block(move || {
        user_service
            .list(filter.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??
    .into_iter()
    .map(|u| u.into())
    .collect::<Vec<UserResponse>>();

    Ok(Json(users))
}

#[api_v2_operation(tags(Users))]
#[post("/users")]
pub async fn create_user(
    Json(request): Json<CreateUserRequest>,
    user_service: Data<UserService>,
) -> actix_web::Result<Json<UserResponse>> {
    let new_user: UserResponse = web::block(move || {
        user_service
            .create(request.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??
    .into();

    Ok(Json(new_user))
}

#[api_v2_operation(tags(Users))]
#[post("/users/authenticate")]
pub async fn authenticate_user(
    Json(request): Json<AuthenticateRequest>,
    user_service: Data<UserService>,
) -> actix_web::Result<Json<UserResponse>> {
    let user: UserResponse = web::block(move || {
        user_service
            .authenticate(request.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??
    .into();

    Ok(Json(user))
}

#[api_v2_operation(tags(Users))]
#[get("/users/{id}")]
pub async fn get_user(
    id: Path<i64>,
    user_service: Data<UserService>,
) -> actix_web::Result<Json<UserResponse>> {
    let id = id.into_inner();
    let user: UserResponse =
        web::block(move || user_service.find(id).map_err(|e| Into::<AppError>::into(e)))
            .await??
            .ok_or(AppError::NotFound {
                entity: "User".to_string(),
                id: id.to_string(),
            })?
            .into();

    Ok(Json(user))
}
