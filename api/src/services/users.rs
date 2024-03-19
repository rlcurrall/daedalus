use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use serde::{Deserialize, Serialize};

use crate::database::PooledConnection;
use crate::models::users::{CreateUser, UpdateUser, User, UserQuery};
use crate::result::{AppError, Result};

#[derive(Serialize, Deserialize)]
pub struct UserCredentials {
    pub tenant_id: i32,
    pub email: String,
    pub password: String,
}

pub struct UserService {
    conn: PooledConnection,
}

impl UserService {
    pub fn new(conn: PooledConnection) -> Self {
        Self { conn }
    }

    pub fn find(&mut self, id: i64) -> Result<Option<User>> {
        Ok(User::find(&mut self.conn, id)?)
    }

    pub fn find_by_email_and_tenant(
        &mut self,
        email: String,
        tenant_id: i32,
    ) -> Result<Option<User>> {
        Ok(User::find_by_email_and_tenant(&mut self.conn, email, tenant_id)?.map(|u| u.into()))
    }

    pub fn list(&mut self, filter: UserQuery) -> Result<Vec<User>> {
        Ok(User::list(&mut self.conn, filter)?)
    }

    pub fn create(
        &mut self,
        CreateUser {
            tenant_id,
            email,
            password,
        }: CreateUser,
    ) -> Result<User> {
        let salt = SaltString::generate(&mut OsRng);
        let argon = Argon2::default();
        let password_hash = argon.hash_password(password.as_bytes(), &salt)?.to_string();

        let exists =
            User::find_by_email_and_tenant(&mut self.conn, email.clone(), tenant_id.clone())?
                .is_some();

        match exists {
            false => Ok(User::create(
                &mut self.conn,
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

    pub fn update(&mut self, id: i64, UpdateUser { email, password }: UpdateUser) -> Result<User> {
        let password = match password {
            None => None,
            Some(password) => {
                let salt = SaltString::generate(&mut OsRng);
                let argon = Argon2::default();
                Some(argon.hash_password(password.as_bytes(), &salt)?.to_string())
            }
        };

        Ok(User::update(&mut self.conn, id, UpdateUser { email, password })?.into())
    }

    pub fn authenticate(
        &mut self,
        UserCredentials {
            tenant_id,
            email,
            password,
        }: UserCredentials,
    ) -> Result<User> {
        let user = User::find_by_email_and_tenant(&mut self.conn, email.clone(), tenant_id)?
            .ok_or(AppError::Forbidden {
                cause: "Invalid email or password".to_string(),
            })?;

        let parsed_hash = PasswordHash::new(&user.password).map_err(|e| AppError::ServerError {
            cause: e.to_string(),
        })?;
        let password_match = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        match password_match {
            true => Ok(user.into()),
            false => Err(AppError::Forbidden {
                cause: "Invalid email or password".to_string(),
            }),
        }
    }
}
