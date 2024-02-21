use actix_identity::error::LoginError;
use actix_web::{
    error::{self, BlockingError},
    http::StatusCode,
    HttpResponse,
};
use derive_more::{Display, Error};
use serde_json::json;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Clone, Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = "{}", cause)]
    BadRequest { cause: String },

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "{}", cause)]
    Forbidden { cause: String },

    #[display(fmt = "{} not found with id: {}", entity, id)]
    NotFound { entity: String, id: String },

    #[display(fmt = "{}", cause)]
    ValidationError { cause: String },

    #[display(fmt = "{}", cause)]
    ServerError { cause: String },
}

impl AppError {
    pub fn validation_error<E: ToString>(cause: E) -> AppError {
        AppError::ValidationError {
            cause: cause.to_string(),
        }
    }

    pub fn bad_request<E: ToString>(cause: E) -> AppError {
        AppError::BadRequest {
            cause: cause.to_string(),
        }
    }

    pub fn server_error<E: ToString>(cause: E) -> AppError {
        AppError::ServerError {
            cause: cause.to_string(),
        }
    }

    pub fn not_found(entity: &str, id: &str) -> AppError {
        AppError::NotFound {
            entity: entity.to_string(),
            id: id.to_string(),
        }
    }

    pub fn forbidden<E: ToString>(cause: E) -> AppError {
        AppError::Forbidden {
            cause: cause.to_string(),
        }
    }

    pub fn unauthorized() -> AppError {
        AppError::Unauthorized
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            AppError::ServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound { .. } => StatusCode::NOT_FOUND,
            AppError::Forbidden { .. } => StatusCode::FORBIDDEN,
            AppError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "type": match *self {
                AppError::ValidationError { .. } => "ValidationError",
                AppError::ServerError { .. } => "ServerError",
                AppError::NotFound { .. } => "NotFound",
                AppError::Forbidden { .. } => "Forbidden",
                AppError::BadRequest { .. } => "BadRequest",
                AppError::Unauthorized => "Unauthorized",
            },
            "error": self.to_string()
        })
    }
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.to_json())
    }
}

impl From<argon2::Error> for AppError {
    fn from(err: argon2::Error) -> Self {
        AppError::forbidden(err.to_string())
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AppError::forbidden(err.to_string())
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
            e => AppError::server_error(e.to_string()),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> AppError {
        AppError::server_error(e.to_string())
    }
}

impl From<AppError> for std::io::Error {
    fn from(e: AppError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    }
}

impl From<diesel::r2d2::Error> for AppError {
    fn from(e: diesel::r2d2::Error) -> AppError {
        AppError::server_error(e.to_string())
    }
}

impl From<BlockingError> for AppError {
    fn from(err: BlockingError) -> AppError {
        AppError::server_error(err.to_string())
    }
}

impl From<LoginError> for AppError {
    fn from(_: LoginError) -> AppError {
        AppError::Unauthorized
    }
}

impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> AppError {
        AppError::server_error(err.to_string())
    }
}

impl From<lexopt::Error> for AppError {
    fn from(err: lexopt::Error) -> AppError {
        AppError::server_error(err.to_string())
    }
}

#[derive(Debug, Display)]
pub struct JsonErrorResponse(pub AppError);
pub type JsonResult<T> = std::result::Result<T, JsonErrorResponse>;

impl JsonErrorResponse {
    pub fn to_json(&self) -> serde_json::Value {
        self.0.to_json()
    }
}

impl error::ResponseError for JsonErrorResponse {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.0.status_code()).json(self.to_json())
    }
}

impl From<AppError> for JsonErrorResponse {
    fn from(err: AppError) -> Self {
        JsonErrorResponse(err)
    }
}

impl From<argon2::Error> for JsonErrorResponse {
    fn from(err: argon2::Error) -> Self {
        JsonErrorResponse(AppError::from(err))
    }
}

impl From<argon2::password_hash::Error> for JsonErrorResponse {
    fn from(err: argon2::password_hash::Error) -> Self {
        JsonErrorResponse(AppError::from(err))
    }
}

impl From<diesel::result::Error> for JsonErrorResponse {
    fn from(err: diesel::result::Error) -> Self {
        JsonErrorResponse(AppError::from(err))
    }
}

impl From<std::io::Error> for JsonErrorResponse {
    fn from(err: std::io::Error) -> Self {
        JsonErrorResponse(AppError::from(err))
    }
}

impl From<JsonErrorResponse> for std::io::Error {
    fn from(err: JsonErrorResponse) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
    }
}

impl From<diesel::r2d2::Error> for JsonErrorResponse {
    fn from(err: diesel::r2d2::Error) -> Self {
        JsonErrorResponse(AppError::from(err))
    }
}

impl From<BlockingError> for JsonErrorResponse {
    fn from(err: BlockingError) -> Self {
        JsonErrorResponse(AppError::from(err))
    }
}

impl From<LoginError> for JsonErrorResponse {
    fn from(err: LoginError) -> Self {
        JsonErrorResponse(AppError::from(err))
    }
}

#[derive(Clone, Debug, Display)]
pub struct HtmlErrorResponse(pub AppError);
pub type HtmlResult<T> = std::result::Result<T, HtmlErrorResponse>;

impl HtmlErrorResponse {
    pub fn to_html(&self) -> String {
        format!(
            r#"<html>
                <head>
                    <title>Error</title>
                </head>
                <body>
                    <h1>Error - {}</h1>
                    <p>{}</p>
                </body>
            </html>"#,
            self.0.status_code().as_str(),
            self.0.to_string()
        )
    }
}

impl error::ResponseError for HtmlErrorResponse {
    fn error_response(&self) -> HttpResponse {
        match self.0 {
            AppError::Unauthorized => HttpResponse::Found()
                .append_header(("location", "/login"))
                .finish(),
            _ => HttpResponse::build(self.0.status_code()).body(self.to_html()),
        }
    }
}

impl From<AppError> for HtmlErrorResponse {
    fn from(err: AppError) -> Self {
        HtmlErrorResponse(err)
    }
}

impl From<argon2::Error> for HtmlErrorResponse {
    fn from(err: argon2::Error) -> Self {
        HtmlErrorResponse(AppError::from(err))
    }
}

impl From<argon2::password_hash::Error> for HtmlErrorResponse {
    fn from(err: argon2::password_hash::Error) -> Self {
        HtmlErrorResponse(AppError::from(err))
    }
}

impl From<diesel::result::Error> for HtmlErrorResponse {
    fn from(err: diesel::result::Error) -> Self {
        HtmlErrorResponse(AppError::from(err))
    }
}

impl From<std::io::Error> for HtmlErrorResponse {
    fn from(err: std::io::Error) -> Self {
        HtmlErrorResponse(AppError::from(err))
    }
}

impl From<HtmlErrorResponse> for std::io::Error {
    fn from(err: HtmlErrorResponse) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, err.to_string())
    }
}

impl From<diesel::r2d2::Error> for HtmlErrorResponse {
    fn from(err: diesel::r2d2::Error) -> Self {
        HtmlErrorResponse(AppError::from(err))
    }
}

impl From<BlockingError> for HtmlErrorResponse {
    fn from(err: BlockingError) -> Self {
        HtmlErrorResponse(AppError::from(err))
    }
}

impl From<LoginError> for HtmlErrorResponse {
    fn from(err: LoginError) -> Self {
        HtmlErrorResponse(AppError::from(err))
    }
}
