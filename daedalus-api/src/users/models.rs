use chrono::{DateTime, Utc};
use daedalus_core::users::{CreateUser, User, UserCredentials, UserFilter};
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

use crate::http::request::*;

#[derive(Deserialize, Serialize, Apiv2Schema)]
pub struct UserListRequest {
    pub tenant_id: Option<i32>,
    pub email: Option<String>,
    #[serde(default = "default_bool_true")]
    pub active: bool,
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_page_size")]
    pub page_size: i64,
}

impl From<UserListRequest> for UserFilter {
    fn from(filter: UserListRequest) -> Self {
        let limit = filter.page_size;
        let offset = (filter.page - 1) * limit;
        Self {
            tenant_id: filter.tenant_id,
            email: filter.email,
            active: filter.active,
            limit,
            offset,
        }
    }
}

#[derive(Deserialize, Serialize, Apiv2Schema)]
pub struct CreateUserRequest {
    pub tenant_id: i32,
    pub email: String,
    pub password: String,
}

impl From<CreateUserRequest> for CreateUser {
    fn from(user: CreateUserRequest) -> Self {
        Self {
            tenant_id: user.tenant_id,
            email: user.email,
            password: user.password,
        }
    }
}

#[derive(Deserialize, Serialize, Apiv2Schema)]
pub struct UserResponse {
    pub id: i64,
    pub tenant_id: i32,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            tenant_id: user.tenant_id,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
        }
    }
}

#[derive(Deserialize, Serialize, Apiv2Schema)]
pub struct AuthenticateRequest {
    pub tenant_id: i32,
    pub email: String,
    pub password: String,
}

impl From<AuthenticateRequest> for UserCredentials {
    fn from(request: AuthenticateRequest) -> Self {
        Self {
            tenant_id: request.tenant_id,
            email: request.email,
            password: request.password,
        }
    }
}
