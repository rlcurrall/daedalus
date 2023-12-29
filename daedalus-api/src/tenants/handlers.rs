use actix_web::web::{self, Data, Json, Path, Query};
use daedalus_core::tenants::TenantService;
use paperclip::actix::{api_v2_operation, get, post};

use crate::{
    result::AppError,
    tenants::models::{
        CreateTenantRequest, TenantListRequest, TenantResponse, UpdateTenantRequest,
    },
};

#[api_v2_operation(tags(Tenants))]
#[get("/tenants")]
pub async fn list_tenants(
    Query(query): Query<TenantListRequest>,
    tenant_service: Data<TenantService>,
) -> actix_web::Result<Json<Vec<TenantResponse>>> {
    let tenants = web::block(move || {
        tenant_service
            .list(query.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??
    .into_iter()
    .map(|t| t.into())
    .collect::<Vec<TenantResponse>>();

    Ok(Json(tenants))
}

#[api_v2_operation(tags(Tenants))]
#[post("/tenants")]
pub async fn create_tenant(
    Json(request): Json<CreateTenantRequest>,
    tenant_service: Data<TenantService>,
) -> actix_web::Result<Json<TenantResponse>> {
    let new_tenant: TenantResponse = web::block(move || {
        tenant_service
            .create(request.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??
    .into();

    Ok(Json(new_tenant))
}

#[api_v2_operation(tags(Tenants))]
#[get("/tenants/{id}")]
pub async fn get_tenant(
    id: Path<i32>,
    tenant_service: Data<TenantService>,
) -> actix_web::Result<Json<TenantResponse>> {
    let id = id.into_inner();
    let tenant: TenantResponse = web::block(move || {
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

#[api_v2_operation(tags(Tenants))]
#[post("/tenants/{id}")]
pub async fn update_tenant(
    id: Path<i32>,
    Json(request): Json<UpdateTenantRequest>,
    tenant_service: Data<TenantService>,
) -> actix_web::Result<Json<TenantResponse>> {
    let id = id.into_inner();
    let updated_tenant: TenantResponse = web::block(move || {
        tenant_service
            .update(id, request.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??
    .into();

    Ok(Json(updated_tenant))
}
