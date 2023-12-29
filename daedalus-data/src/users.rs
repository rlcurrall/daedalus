use diesel::{
    helper_types::{AsSelect, Filter, Select},
    prelude::*,
};

use crate::{result::Error, schema::users};

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct CreateUser {
    pub tenant_id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(crate::DB))]
pub struct User {
    pub id: i64,
    pub tenant_id: i32,
    pub email: String,
    pub password: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct UserFilter {
    pub tenant_id: Option<i32>,
    pub email: Option<String>,
    pub active: bool,
    pub limit: i64,
    pub offset: i64,
}

type Query = Select<users::table, AsSelect<User, crate::DB>>;
type ById = Filter<Query, diesel::dsl::Eq<users::id, i64>>;
type ByEmailAndTenantId = Filter<
    Filter<Query, diesel::dsl::Eq<users::email, String>>,
    diesel::dsl::Eq<users::tenant_id, i32>,
>;

impl User {
    pub fn find(conn: &mut crate::DbConnection, id: i64) -> Result<Option<User>, Error> {
        Ok(User::by_id(id).get_result(conn).optional()?)
    }

    pub fn find_by_email_and_tenant(
        conn: &mut crate::DbConnection,
        email: String,
        tenant_id: i32,
    ) -> Result<Option<User>, Error> {
        Ok(User::by_email_and_tenant(email, tenant_id)
            .get_result(conn)
            .optional()?)
    }

    pub fn create(conn: &mut crate::DbConnection, user: CreateUser) -> Result<User, Error> {
        Ok(diesel::insert_into(users::table)
            .values(&user)
            .returning(User::as_returning())
            .get_result(conn)?)
    }

    pub fn list(
        conn: &mut crate::DbConnection,
        UserFilter {
            tenant_id,
            email,
            active,
            limit,
            offset,
        }: UserFilter,
    ) -> Result<Vec<User>, Error> {
        let mut query = users::table.into_boxed::<crate::DB>();

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
            .limit(limit)
            .offset(offset)
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
