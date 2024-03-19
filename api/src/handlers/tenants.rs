use actix_web::web::{block, Data, Json, Path, Query};

use crate::database::PoolManager;
use crate::middleware::bearer::UserClaims;
use crate::models::common::Paginated;
use crate::result::{AppError, JsonResult};
use crate::tenants::{CreateTenant, Tenant, TenantQuery, UpdateTenant};

pub async fn list(
    _: UserClaims,
    Query(query): Query<TenantQuery>,
    pool: Data<PoolManager>,
) -> JsonResult<Json<Paginated<Tenant>>> {
    let tenants = block(move || {
        let mut conn = pool.get()?;
        Tenant::paginate(&mut conn, query.into())
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
        let mut conn = pool.get()?;
        Tenant::create(&mut conn, request.into())
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
        let mut conn = pool.get()?;
        Tenant::find(&mut conn, id)?.ok_or(AppError::NotFound {
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
        let mut conn = pool.get()?;
        Tenant::update(&mut conn, id, request.into())
    })
    .await??;

    Ok(Json(updated_tenant))
}
