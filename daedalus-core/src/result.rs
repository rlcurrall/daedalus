pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    PasswordError(String),
    DatabaseError {
        kind: DatabaseErrorKind,
        cause: String,
    },
}

pub enum DatabaseErrorKind {
    NotFound,
    UniqueViolation,
    ForeignKeyViolation,
    NotNullViolation,
    CheckViolation,
    ConnectionError,
}

impl From<argon2::Error> for Error {
    fn from(err: argon2::Error) -> Self {
        Self::PasswordError(err.to_string())
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(err: argon2::password_hash::Error) -> Self {
        Self::PasswordError(err.to_string())
    }
}

impl From<daedalus_data::result::Error> for Error {
    fn from(err: daedalus_data::result::Error) -> Self {
        Self::DatabaseError {
            kind: match err {
                daedalus_data::result::Error::DatabaseError(_) => DatabaseErrorKind::NotFound,
                daedalus_data::result::Error::NotFound => DatabaseErrorKind::NotFound,
                daedalus_data::result::Error::UniqueViolation(_) => {
                    DatabaseErrorKind::UniqueViolation
                }
                daedalus_data::result::Error::ForeignKeyViolation(_) => {
                    DatabaseErrorKind::ForeignKeyViolation
                }
                daedalus_data::result::Error::NotNullViolation(_) => {
                    DatabaseErrorKind::NotNullViolation
                }
                daedalus_data::result::Error::CheckViolation(_) => {
                    DatabaseErrorKind::CheckViolation
                }
            },
            cause: err.to_string(),
        }
    }
}
