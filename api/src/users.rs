use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use diesel::helper_types::{AsSelect, Filter, Select};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use tsync::tsync;

use crate::database::{schema::users, DbConnection, DB};
use crate::models::common::Paginated;
use crate::models::defaults::{default_bool, default_i64};
use crate::result::AppError;

#[derive(Clone, Debug, Deserialize, Insertable, Serialize)]
#[tsync]
#[diesel(table_name = users)]
pub struct CreateUser {
    pub tenant_id: i32,
    pub email: String,
    pub password: String,
}

#[derive(AsChangeset, Clone, Debug, Deserialize, Serialize)]
#[tsync]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Queryable, Selectable, Serialize)]
#[tsync]
#[diesel(table_name = users)]
pub struct User {
    pub id: i64,
    pub tenant_id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[tsync]
pub struct UserQuery {
    pub tenant_id: Option<i32>,
    pub email: Option<String>,
    #[serde(default = "default_bool::<true>")]
    pub active: bool,
    #[serde(default = "default_i64::<1>")]
    pub page: i64,
    #[serde(default = "default_i64::<10>")]
    pub page_size: i64,
}

#[derive(Serialize, Deserialize)]
pub struct UserCredentials {
    pub tenant_id: i32,
    pub email: String,
    pub password: String,
}

type Query = Select<users::table, AsSelect<User, DB>>;
type ById = Filter<Query, diesel::dsl::Eq<users::id, i64>>;
type ByEmailAndTenantId = Filter<
    Filter<Query, diesel::dsl::Eq<users::email, String>>,
    diesel::dsl::Eq<users::tenant_id, i32>,
>;

impl User {
    pub fn find(conn: &mut DbConnection, id: i64) -> Result<Option<User>, AppError> {
        Ok(User::by_id(id).get_result(conn).optional()?)
    }

    pub fn find_by_email_and_tenant(
        conn: &mut DbConnection,
        email: String,
        tenant_id: i32,
    ) -> Result<Option<User>, AppError> {
        Ok(User::by_email_and_tenant(email, tenant_id)
            .get_result(conn)
            .optional()?)
    }

    pub fn create(
        conn: &mut DbConnection,
        CreateUser {
            tenant_id,
            email,
            password,
        }: CreateUser,
    ) -> Result<User, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon = Argon2::default();
        let password = argon.hash_password(password.as_bytes(), &salt)?.to_string();

        if Self::find_by_email_and_tenant(conn, email.clone(), tenant_id.clone())?.is_some() {
            return Err(AppError::bad_request("User already exists"));
        }

        Ok(diesel::insert_into(users::table)
            .values(&CreateUser {
                tenant_id,
                email,
                password,
            })
            .returning(User::as_returning())
            .get_result(conn)?)
    }

    pub fn update(
        conn: &mut DbConnection,
        id: i64,
        UpdateUser { email, password }: UpdateUser,
    ) -> Result<User, AppError> {
        let password = match password {
            None => None,
            Some(password) => {
                let salt = SaltString::generate(&mut OsRng);
                let argon = Argon2::default();
                Some(argon.hash_password(password.as_bytes(), &salt)?.to_string())
            }
        };

        Ok(diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(&UpdateUser { email, password })
            .returning(User::as_returning())
            .get_result(conn)?)
    }

    pub fn list(
        conn: &mut DbConnection,
        UserQuery {
            tenant_id,
            email,
            active,
            page,
            page_size,
        }: UserQuery,
    ) -> Result<Vec<User>, AppError> {
        let mut query = users::table.into_boxed::<DB>();

        if let Some(tenant_id) = tenant_id {
            query = query.filter(users::tenant_id.eq(tenant_id));
        }

        if let Some(email) = email {
            query = query.filter(users::email.eq(email));
        }

        query = match active {
            true => query.filter(users::deleted_at.is_null()),
            false => query.filter(users::deleted_at.is_not_null()),
        };

        Ok(query
            .select(User::as_select())
            .limit(page_size)
            .offset(page_size * (page - 1))
            .get_results(conn)?)
    }

    pub fn count(
        conn: &mut DbConnection,
        UserQuery {
            tenant_id,
            email,
            active,
            ..
        }: UserQuery,
    ) -> Result<i64, AppError> {
        let mut query = users::table.into_boxed::<DB>();

        if let Some(tenant_id) = tenant_id {
            query = query.filter(users::tenant_id.eq(tenant_id));
        }

        if let Some(email) = email {
            query = query.filter(users::email.eq(email));
        }

        query = match active {
            true => query.filter(users::deleted_at.is_null()),
            false => query.filter(users::deleted_at.is_not_null()),
        };

        Ok(query.count().get_result(conn)?)
    }

    pub fn paginate(
        conn: &mut DbConnection,
        filter: UserQuery,
    ) -> Result<Paginated<User>, AppError> {
        let total = User::count(conn, filter.clone())?;
        let data = User::list(conn, filter.clone())?;

        Ok(Paginated {
            total,
            page: filter.page,
            per_page: filter.page_size,
            data,
        })
    }

    pub fn authenticate(
        conn: &mut DbConnection,
        UserCredentials {
            tenant_id,
            email,
            password,
        }: UserCredentials,
    ) -> Result<User, AppError> {
        let user = Self::find_by_email_and_tenant(conn, email.clone(), tenant_id)?.ok_or(
            AppError::Forbidden {
                cause: "Invalid email or password".to_string(),
            },
        )?;

        let parsed_hash = PasswordHash::new(&user.password).map_err(|e| AppError::ServerError {
            cause: e.to_string(),
        })?;
        let password_match = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        match password_match {
            true => Ok(user),
            false => Err(AppError::Forbidden {
                cause: "Invalid email or password".to_string(),
            }),
        }
    }

    pub(crate) fn query() -> Query {
        users::table.select(User::as_select())
    }

    pub(crate) fn by_id(id: i64) -> ById {
        User::query().filter(users::id.eq(id))
    }

    pub(crate) fn by_email_and_tenant(email: String, tenant_id: i32) -> ByEmailAndTenantId {
        User::query()
            .filter(users::email.eq(email))
            .filter(users::tenant_id.eq(tenant_id))
    }
}
