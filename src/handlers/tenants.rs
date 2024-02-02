use actix_web::web::{block, Data, Json, Path, Query};

use crate::models::tenants::{CreateTenant, Tenant, TenantQuery, UpdateTenant};
use crate::result::AppError;
use crate::services::tenants::TenantService;

pub async fn list(
    Query(query): Query<TenantQuery>,
    tenant_service: Data<TenantService>,
) -> actix_web::Result<Json<Vec<Tenant>>> {
    let tenants = block(move || {
        tenant_service
            .list(query.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??
    .into_iter()
    .map(|t| t.into())
    .collect::<Vec<Tenant>>();

    Ok(Json(tenants))
}

pub async fn create(
    Json(request): Json<CreateTenant>,
    tenant_service: Data<TenantService>,
) -> actix_web::Result<Json<Tenant>> {
    let new_tenant: Tenant = block(move || {
        tenant_service
            .create(request.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??
    .into();

    Ok(Json(new_tenant))
}

pub async fn find(
    id: Path<i32>,
    tenant_service: Data<TenantService>,
) -> actix_web::Result<Json<Tenant>> {
    let id = id.into_inner();
    let tenant: Tenant = block(move || {
        tenant_service
            .find(id)
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??
    .ok_or(AppError::NotFound {
        entity: "Tenant".to_string(),
        id: id.to_string(),
    })?
    .into();

    Ok(Json(tenant))
}

pub async fn update(
    id: Path<i32>,
    Json(request): Json<UpdateTenant>,
    tenant_service: Data<TenantService>,
) -> actix_web::Result<Json<Tenant>> {
    let id = id.into_inner();
    let updated_tenant: Tenant = block(move || {
        tenant_service
            .update(id, request.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??
    .into();

    Ok(Json(updated_tenant))
}
