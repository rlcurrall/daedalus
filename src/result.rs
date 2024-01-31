use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use serde_json::json;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = "{}", cause)]
    ValidationError { cause: String },

    #[display(fmt = "{}", cause)]
    BadRequest { cause: String },

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
                AppError::BadRequest { .. } => "BadRequest",
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
            AppError::BadRequest { .. } => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<argon2::Error> for AppError {
    fn from(err: argon2::Error) -> Self {
        AppError::Forbidden {
            cause: err.to_string(),
        }
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AppError::Forbidden {
            cause: err.to_string(),
        }
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(e: diesel::result::Error) -> AppError {
        match e {
            diesel::result::Error::NotFound => AppError::NotFound {
                entity: "Entity".to_string(),
                id: "Unknown".to_string(),
            },
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                info,
            ) => AppError::Forbidden {
                cause: info.message().to_string(),
            },
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::ForeignKeyViolation,
                info,
            ) => AppError::BadRequest {
                cause: info.message().to_string(),
            },
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::NotNullViolation,
                info,
            ) => AppError::BadRequest {
                cause: info.message().to_string(),
            },
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::CheckViolation,
                info,
            ) => AppError::BadRequest {
                cause: info.message().to_string(),
            },
            diesel::result::Error::DatabaseError(_, info) => AppError::ServerError {
                cause: info.message().to_string(),
            },
            e => AppError::ServerError {
                cause: e.to_string(),
            },
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> AppError {
        AppError::ServerError {
            cause: e.to_string(),
        }
    }
}

impl From<AppError> for std::io::Error {
    fn from(e: AppError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    }
}
