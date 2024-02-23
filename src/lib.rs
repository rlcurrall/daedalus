#![recursion_limit = "256"]

use std::future::{ready, Ready};

use actix_identity::Identity;
use actix_web::{FromRequest, HttpRequest};
use result::AppError;

pub mod config;
pub mod database;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod result;
pub mod routes;
pub mod server;
pub mod services;
pub mod tmpl;

pub struct UserId(i64);

impl FromRequest for UserId {
    type Error = AppError;
    type Future = Ready<Result<UserId, AppError>>;

    fn from_request(req: &HttpRequest, pl: &mut actix_web::dev::Payload) -> Self::Future {
        if let Ok(id) = Identity::from_request(req, pl).into_inner() {
            if let Ok(id) = id.id() {
                if let Ok(id) = id.parse::<i64>() {
                    return ready(Ok(UserId(id)));
                }
            }
        }

        ready(Err(AppError::Unauthorized))
    }
}
