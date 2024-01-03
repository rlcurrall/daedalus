use crate::{
    database::{DbPool, PooledConnection},
    models::tenants::{CreateTenant, Tenant, TenantQuery, UpdateTenant},
    result::{AppError, Result},
};

#[derive(Clone)]
pub struct TenantService {
    pool: DbPool,
}

impl TenantService {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn create(&self, tenant: CreateTenant) -> Result<Tenant> {
        let mut conn = self.get_connection()?;
        Ok(Tenant::create(&mut conn, tenant.into())?.into())
    }

    pub fn update(&self, id: i32, tenant: UpdateTenant) -> Result<Tenant> {
        let mut conn = self.get_connection()?;
        Ok(Tenant::update(&mut conn, id, tenant.into())?.into())
    }

    pub fn delete(&self, id: i32) -> Result<Tenant> {
        let mut conn = self.get_connection()?;
        Ok(Tenant::delete(&mut conn, id)?.into())
    }

    pub fn find(&self, id: i32) -> Result<Option<Tenant>> {
        let mut conn = self.get_connection()?;
        Ok(Tenant::find(&mut conn, id)?.map(|t| t.into()))
    }

    pub fn find_by_name(&self, name: String) -> Result<Option<Tenant>> {
        let mut conn = self.get_connection()?;
        Ok(Tenant::find_by_name(&mut conn, name)?.map(|t| t.into()))
    }

    pub fn list(&self, query: TenantQuery) -> Result<Vec<Tenant>> {
        let mut conn = self.get_connection()?;
        Ok(Tenant::list(&mut conn, query.into())?
            .into_iter()
            .map(|t| t.into())
            .collect())
    }

    fn get_connection(&self) -> Result<PooledConnection> {
        self.pool.get().map_err(|e| AppError::ServerError {
            cause: e.to_string(),
        })
    }
}
