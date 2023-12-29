use actix_web::{error, http::StatusCode, HttpResponse};
use daedalus_core::result::{DatabaseErrorKind, Error as CoreError};
use derive_more::{Display, Error};
use serde_json::json;

#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = "{}", cause)]
    ValidationError { cause: String },

    #[display(fmt = "Internal Server Error")]
    ServerError { cause: String },

    #[display(fmt = "{} not found with id: {}", entity, id)]
    NotFound { entity: String, id: String },

    #[display(fmt = "{}", cause)]
    Forbidden { cause: String },
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(json!({
            "type": match *self {
                AppError::ValidationError { .. } => "ValidationError",
                AppError::ServerError { .. } => "ServerError",
                AppError::NotFound { .. } => "NotFound",
                AppError::Forbidden { .. } => "Forbidden",
            },
            "error": self.to_string()
        }))
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            AppError::ServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound { .. } => StatusCode::NOT_FOUND,
            AppError::Forbidden { .. } => StatusCode::FORBIDDEN,
        }
    }
}

impl From<CoreError> for AppError {
    fn from(e: CoreError) -> AppError {
        match e {
            CoreError::PasswordError(cause) => AppError::Forbidden { cause },
            CoreError::DatabaseError { kind, cause } => match kind {
                DatabaseErrorKind::NotFound => AppError::NotFound {
                    entity: "User".to_string(),
                    id: "id".to_string(),
                },
                DatabaseErrorKind::UniqueViolation => AppError::ValidationError {
                    cause: "Duplicate record".to_string(),
                },
                _ => AppError::ServerError { cause },
            },
        }
    }
}
