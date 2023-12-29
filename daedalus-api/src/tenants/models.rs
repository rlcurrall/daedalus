use chrono::{DateTime, Utc};
use daedalus_core::tenants::{CreateTenant, Tenant, TenantQuery, UpdateTenant};
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

use crate::http::request::*;

#[derive(Deserialize, Serialize, Apiv2Schema)]
pub struct TenantListRequest {
    pub name: Option<String>,
    #[serde(default = "default_bool_true")]
    pub active: bool,
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_page_size")]
    pub page_size: i64,
}

impl From<TenantListRequest> for TenantQuery {
    fn from(filter: TenantListRequest) -> Self {
        let limit = filter.page_size;
        let offset = (filter.page - 1) * limit;
        Self {
            name: filter.name,
            active: filter.active,
            limit,
            offset,
        }
    }
}

#[derive(Deserialize, Serialize, Apiv2Schema)]
pub struct CreateTenantRequest {
    pub name: String,
}

impl From<CreateTenantRequest> for CreateTenant {
    fn from(tenant: CreateTenantRequest) -> Self {
        Self { name: tenant.name }
    }
}

#[derive(Deserialize, Serialize, Apiv2Schema)]
pub struct UpdateTenantRequest {
    pub name: Option<String>,
}

impl From<UpdateTenantRequest> for UpdateTenant {
    fn from(tenant: UpdateTenantRequest) -> Self {
        Self { name: tenant.name }
    }
}

#[derive(Deserialize, Serialize, Apiv2Schema)]
pub struct TenantResponse {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<Tenant> for TenantResponse {
    fn from(tenant: Tenant) -> Self {
        Self {
            id: tenant.id,
            name: tenant.name,
            created_at: tenant.created_at,
            updated_at: tenant.updated_at,
            deleted_at: tenant.deleted_at,
        }
    }
}
