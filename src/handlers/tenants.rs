use actix_web::web::{block, Data, Json, Path, Query};

use crate::database::PoolManager;
use crate::middleware::bearer::UserClaims;
use crate::models::tenants::{CreateTenant, Tenant, TenantQuery, UpdateTenant};
use crate::result::AppError;
use crate::result::JsonResult;
use crate::services::tenants::TenantService;

pub async fn list(
    _: UserClaims,
    Query(query): Query<TenantQuery>,
    pool: Data<PoolManager>,
) -> JsonResult<Json<Vec<Tenant>>> {
    let tenants = block(move || {
        let conn = pool.get()?;
        TenantService::new(conn).list(query.into())
    })
    .await??;

    Ok(Json(tenants))
}

pub async fn create(
    _: UserClaims,
    Json(request): Json<CreateTenant>,
    pool: Data<PoolManager>,
) -> JsonResult<Json<Tenant>> {
    let new_tenant: Tenant = block(move || {
        let conn = pool.get()?;
        TenantService::new(conn).create(request.into())
    })
    .await??;

    Ok(Json(new_tenant))
}

pub async fn find(
    _: UserClaims,
    id: Path<i32>,
    pool: Data<PoolManager>,
) -> JsonResult<Json<Tenant>> {
    let id = id.into_inner();
    let tenant: Tenant = block(move || {
        let conn = pool.get()?;
        TenantService::new(conn)
            .find(id)?
            .ok_or(AppError::NotFound {
                entity: "Tenant".to_string(),
                id: id.to_string(),
            })
    })
    .await??;

    Ok(Json(tenant))
}

pub async fn update(
    _: UserClaims,
    id: Path<i32>,
    Json(request): Json<UpdateTenant>,
    pool: Data<PoolManager>,
) -> JsonResult<Json<Tenant>> {
    let id = id.into_inner();
    let updated_tenant: Tenant = block(move || {
        let conn = pool.get()?;
        TenantService::new(conn).update(id, request.into())
    })
    .await??;

    Ok(Json(updated_tenant))
}
