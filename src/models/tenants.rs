use chrono::{DateTime, Utc};
use diesel::{
    helper_types::{AsSelect, Filter, Select},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use tsync::tsync;

use super::defaults::{default_bool, default_i64};
use crate::{
    database::{schema::tenants, DbConnection, DB},
    result::Result,
};

#[derive(Clone, Debug, Deserialize, Queryable, Selectable, Serialize)]
#[tsync]
#[diesel(table_name = tenants)]
pub struct Tenant {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[tsync]
pub struct TenantQuery {
    pub name: Option<String>,
    #[serde(default = "default_bool::<true>")]
    pub active: bool,
    #[serde(default = "default_i64::<1>")]
    pub page: i64,
    #[serde(default = "default_i64::<10>")]
    pub page_size: i64,
}

#[derive(Clone, Debug, Deserialize, Insertable, Serialize)]
#[tsync]
#[diesel(table_name = tenants)]
pub struct CreateTenant {
    pub name: String,
}

#[derive(AsChangeset, Clone, Debug, Deserialize, Serialize)]
#[tsync]
#[diesel(table_name = tenants)]
pub struct UpdateTenant {
    pub name: Option<String>,
}

type Query = Select<tenants::table, AsSelect<Tenant, DB>>;
type ById = Filter<Query, diesel::dsl::Eq<tenants::id, i32>>;
type ByName = Filter<Query, diesel::dsl::Eq<tenants::name, String>>;

impl Tenant {
    pub fn find(conn: &mut DbConnection, id: i32) -> Result<Option<Tenant>> {
        Ok(Tenant::by_id(id).get_result(conn).optional()?)
    }

    pub fn find_by_name(conn: &mut DbConnection, name: String) -> Result<Option<Tenant>> {
        Ok(Tenant::by_name(name).get_result(conn).optional()?)
    }

    pub fn create(conn: &mut DbConnection, tenant: CreateTenant) -> Result<Tenant> {
        Ok(diesel::insert_into(tenants::table)
            .values(&tenant)
            .returning(Tenant::as_returning())
            .get_result(conn)?)
    }

    pub fn update(conn: &mut DbConnection, id: i32, tenant: UpdateTenant) -> Result<Tenant> {
        Ok(diesel::update(tenants::table)
            .filter(tenants::id.eq(id))
            .set(&tenant)
            .returning(Tenant::as_returning())
            .get_result(conn)?)
    }

    pub fn delete(conn: &mut DbConnection, id: i32) -> Result<Tenant> {
        Ok(diesel::update(tenants::table)
            .filter(tenants::id.eq(id))
            .set(tenants::deleted_at.eq(Utc::now()))
            .returning(Tenant::as_returning())
            .get_result(conn)?)
    }

    pub fn list(
        conn: &mut DbConnection,
        TenantQuery {
            name,
            active,
            page,
            page_size,
        }: TenantQuery,
    ) -> Result<Vec<Tenant>> {
        let mut query = tenants::table.into_boxed::<DB>();

        if let Some(name) = name {
            query = query.filter(tenants::name.eq(name));
        }

        if active {
            query = query.filter(tenants::deleted_at.is_null());
        } else {
            query = query.filter(tenants::deleted_at.is_not_null());
        }

        Ok(query
            .limit(page_size)
            .offset(page_size * (page - 1))
            .load::<Tenant>(conn)?)
    }

    fn query() -> Query {
        tenants::table.select(Tenant::as_select())
    }

    fn by_id(id: i32) -> ById {
        Tenant::query().filter(tenants::id.eq(id))
    }

    fn by_name(name: String) -> ByName {
        Tenant::query().filter(tenants::name.eq(name))
    }
}
