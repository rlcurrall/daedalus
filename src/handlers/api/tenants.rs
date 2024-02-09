use actix_web::{
    web::{block, Data, Json, Path, Query},
    Result,
};

use crate::models::tenants::{CreateTenant, Tenant, TenantQuery, UpdateTenant};
use crate::result::AppError;
use crate::services::tenants::TenantService;
use crate::{database::PoolManager, UserId};

pub async fn list(
    _: UserId,
    Query(query): Query<TenantQuery>,
    pool: Data<PoolManager>,
) -> Result<Json<Vec<Tenant>>> {
    let tenants = block(move || {
        let conn = pool.get()?;
        TenantService::new(conn)
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
    _: UserId,
    Json(request): Json<CreateTenant>,
    pool: Data<PoolManager>,
) -> Result<Json<Tenant>> {
    let new_tenant: Tenant = block(move || {
        let conn = pool.get()?;
        TenantService::new(conn)
            .create(request.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??
    .into();

    Ok(Json(new_tenant))
}

pub async fn find(_: UserId, id: Path<i32>, pool: Data<PoolManager>) -> Result<Json<Tenant>> {
    let id = id.into_inner();
    let tenant: Tenant = block(move || {
        let conn = pool.get()?;
        TenantService::new(conn)
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
    _: UserId,
    id: Path<i32>,
    Json(request): Json<UpdateTenant>,
    pool: Data<PoolManager>,
) -> Result<Json<Tenant>> {
    let id = id.into_inner();
    let updated_tenant: Tenant = block(move || {
        let conn = pool.get()?;
        TenantService::new(conn)
            .update(id, request.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??
    .into();

    Ok(Json(updated_tenant))
}
