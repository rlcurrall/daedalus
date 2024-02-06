use diesel::{
    helper_types::{AsSelect, Filter, Select},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use tsync::tsync;

use super::defaults::{default_bool, default_i64};
use crate::{
    database::{schema::users, DbConnection, DB},
    result::AppError,
};

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

    pub fn create(conn: &mut DbConnection, user: CreateUser) -> Result<User, AppError> {
        Ok(diesel::insert_into(users::table)
            .values(&user)
            .returning(User::as_returning())
            .get_result(conn)?)
    }

    pub fn update(conn: &mut DbConnection, id: i64, user: UpdateUser) -> Result<User, AppError> {
        Ok(diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(&user)
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
