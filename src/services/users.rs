use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use serde::{Deserialize, Serialize};

use crate::{
    database::DbPool,
    models::users::{CreateUser, User, UserQuery},
};
use crate::{
    database::PooledConnection,
    result::{AppError, Result},
};

#[derive(Serialize, Deserialize)]
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

    pub fn list(&self, filter: UserQuery) -> Result<Vec<User>> {
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
            true => Err(AppError::BadRequest {
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
            AppError::NotFound {
                entity: "User".to_string(),
                id: email.clone(),
            },
        )?;

        let parsed_hash = PasswordHash::new(&user.password).map_err(|e| AppError::Forbidden {
            cause: e.to_string(),
        })?;
        let password_match = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        match password_match {
            true => Ok(user.into()),
            false => Err(AppError::Forbidden {
                cause: "Invalid password".to_string(),
            }),
        }
    }

    fn get_connection(&self) -> Result<PooledConnection> {
        self.pool.get().map_err(|e| AppError::ServerError {
            cause: e.to_string(),
        })
    }
}
