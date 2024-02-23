use crate::database::PooledConnection;
use crate::models::tenants::{CreateTenant, Tenant, TenantQuery, UpdateTenant};
use crate::result::Result;

pub struct TenantService {
    conn: PooledConnection,
}

impl TenantService {
    pub fn new(conn: PooledConnection) -> Self {
        Self { conn }
    }

    pub fn create(&mut self, tenant: CreateTenant) -> Result<Tenant> {
        Ok(Tenant::create(&mut self.conn, tenant.into())?.into())
    }

    pub fn update(&mut self, id: i32, tenant: UpdateTenant) -> Result<Tenant> {
        Ok(Tenant::update(&mut self.conn, id, tenant.into())?.into())
    }

    pub fn delete(&mut self, id: i32) -> Result<Tenant> {
        Ok(Tenant::delete(&mut self.conn, id)?.into())
    }

    pub fn find(&mut self, id: i32) -> Result<Option<Tenant>> {
        Ok(Tenant::find(&mut self.conn, id)?.map(|t| t.into()))
    }

    pub fn find_by_name(&mut self, name: String) -> Result<Option<Tenant>> {
        Ok(Tenant::find_by_name(&mut self.conn, name)?.map(|t| t.into()))
    }

    pub fn list(&mut self, query: TenantQuery) -> Result<Vec<Tenant>> {
        Ok(Tenant::list(&mut self.conn, query.into())?
            .into_iter()
            .map(|t| t.into())
            .collect())
    }
}
