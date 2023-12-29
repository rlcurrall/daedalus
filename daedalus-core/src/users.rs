use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use daedalus_data::DbPool;

use crate::result::{DatabaseErrorKind, Error, Result};

pub use daedalus_data::users::{CreateUser, User, UserFilter};

pub struct UserCredentials {
    pub tenant_id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Clone)]
pub struct UserService {
    pool: DbPool,
}

impl UserService {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn find(&self, id: i64) -> Result<Option<User>> {
        let mut conn = self.get_connection()?;

        Ok(User::find(&mut conn, id)?)
    }

    pub fn find_by_email_and_tenant(&self, email: String, tenant_id: i32) -> Result<Option<User>> {
        let mut conn = self.get_connection()?;

        Ok(User::find_by_email_and_tenant(&mut conn, email, tenant_id)?.map(|u| u.into()))
    }

    pub fn list(&self, filter: UserFilter) -> Result<Vec<User>> {
        let mut conn = self.get_connection()?;

        Ok(User::list(&mut conn, filter)?)
    }

    pub fn create(
        &self,
        CreateUser {
            tenant_id,
            email,
            password,
        }: CreateUser,
    ) -> Result<User> {
        let mut conn = self.get_connection()?;

        let salt = SaltString::generate(&mut OsRng);
        let argon = Argon2::default();
        let password_hash = argon.hash_password(password.as_bytes(), &salt)?.to_string();

        let exists =
            User::find_by_email_and_tenant(&mut conn, email.clone(), tenant_id.clone())?.is_some();

        match exists {
            false => Ok(User::create(
                &mut conn,
                CreateUser {
                    tenant_id,
                    email,
                    password: password_hash,
                },
            )?
            .into()),
            true => Err(Error::DatabaseError {
                kind: DatabaseErrorKind::UniqueViolation,
                cause: format!("User with email {} already exists", email),
            }),
        }
    }

    pub fn authenticate(
        &self,
        UserCredentials {
            tenant_id,
            email,
            password,
        }: UserCredentials,
    ) -> Result<User> {
        let mut conn = self.get_connection()?;

        let user = User::find_by_email_and_tenant(&mut conn, email.clone(), tenant_id)?.ok_or(
            Error::DatabaseError {
                kind: DatabaseErrorKind::NotFound,
                cause: format!("User with email {} not found", email),
            },
        )?;

        let parsed_hash =
            PasswordHash::new(&user.password).map_err(|e| Error::PasswordError(e.to_string()))?;
        let password_match = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        match password_match {
            true => Ok(user.into()),
            false => Err(Error::PasswordError("Invalid password".to_string())),
        }
    }

    fn get_connection(&self) -> Result<daedalus_data::PooledConnection> {
        self.pool.get().map_err(|e| Error::DatabaseError {
            kind: DatabaseErrorKind::ConnectionError,
            cause: e.to_string(),
        })
    }
}
