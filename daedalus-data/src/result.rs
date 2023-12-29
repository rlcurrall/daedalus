use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    DatabaseError(String),
    NotFound,
    UniqueViolation(String),
    ForeignKeyViolation(String),
    NotNullViolation(String),
    CheckViolation(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            Error::NotFound => write!(f, "Not found"),
            Error::UniqueViolation(msg) => write!(f, "Unique violation: {}", msg),
            Error::ForeignKeyViolation(msg) => write!(f, "Foreign key violation: {}", msg),
            Error::NotNullViolation(msg) => write!(f, "Not null violation: {}", msg),
            Error::CheckViolation(msg) => write!(f, "Check violation: {}", msg),
        }
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Error {
        match e {
            diesel::result::Error::NotFound => Error::NotFound,
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                info,
            ) => Error::UniqueViolation(info.message().to_string()),
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::ForeignKeyViolation,
                info,
            ) => Error::ForeignKeyViolation(info.message().to_string()),
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::NotNullViolation,
                info,
            ) => Error::NotNullViolation(info.message().to_string()),
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::CheckViolation,
                info,
            ) => Error::CheckViolation(info.message().to_string()),
            diesel::result::Error::DatabaseError(_, info) => {
                Error::DatabaseError(info.message().to_string())
            }
            e => Error::DatabaseError(e.to_string()),
        }
    }
}
