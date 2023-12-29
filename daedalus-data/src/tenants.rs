use chrono::{DateTime, Utc};
use diesel::{
    helper_types::{AsSelect, Filter, Select},
    prelude::*,
};

use crate::{result::Result, schema::tenants};

#[derive(Queryable, Selectable)]
#[diesel(table_name = tenants)]
#[diesel(check_for_backend(crate::DB))]
pub struct Tenant {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

pub struct TenantQuery {
    pub name: Option<String>,
    pub active: bool,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Insertable)]
#[diesel(table_name = tenants)]
pub struct CreateTenant {
    pub name: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = tenants)]
pub struct UpdateTenant {
    pub name: Option<String>,
}

type Query = Select<tenants::table, AsSelect<Tenant, crate::DB>>;
type ById = Filter<Query, diesel::dsl::Eq<tenants::id, i32>>;
type ByName = Filter<Query, diesel::dsl::Eq<tenants::name, String>>;

impl Tenant {
    pub fn find(conn: &mut crate::DbConnection, id: i32) -> Result<Option<Tenant>> {
        Ok(Tenant::by_id(id).get_result(conn).optional()?)
    }

    pub fn find_by_name(conn: &mut crate::DbConnection, name: String) -> Result<Option<Tenant>> {
        Ok(Tenant::by_name(name).get_result(conn).optional()?)
    }

    pub fn create(conn: &mut crate::DbConnection, tenant: CreateTenant) -> Result<Tenant> {
        Ok(diesel::insert_into(tenants::table)
            .values(&tenant)
            .returning(Tenant::as_returning())
            .get_result(conn)?)
    }

    pub fn update(conn: &mut crate::DbConnection, id: i32, tenant: UpdateTenant) -> Result<Tenant> {
        Ok(diesel::update(tenants::table)
            .filter(tenants::id.eq(id))
            .set(&tenant)
            .returning(Tenant::as_returning())
            .get_result(conn)?)
    }

    pub fn delete(conn: &mut crate::DbConnection, id: i32) -> Result<Tenant> {
        Ok(diesel::update(tenants::table)
            .filter(tenants::id.eq(id))
            .set(tenants::deleted_at.eq(Utc::now()))
            .returning(Tenant::as_returning())
            .get_result(conn)?)
    }

    pub fn list(
        conn: &mut crate::DbConnection,
        TenantQuery {
            name,
            active,
            limit,
            offset,
        }: TenantQuery,
    ) -> Result<Vec<Tenant>> {
        let mut query = tenants::table.into_boxed::<crate::DB>();

        if let Some(name) = name {
            query = query.filter(tenants::name.eq(name));
        }

        if active {
            query = query.filter(tenants::deleted_at.is_null());
        } else {
            query = query.filter(tenants::deleted_at.is_not_null());
        }

        query
            .offset(offset)
            .limit(limit)
            .load::<Tenant>(conn)
            .map_err(Into::into)
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
