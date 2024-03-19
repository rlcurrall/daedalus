use actix_web::web::{block, Data, Json, Path, Query};

use crate::database::PoolManager;
use crate::middleware::bearer::UserClaims;
use crate::models::common::Paginated;
use crate::models::workflows::{NewWorkflow, UpdateWorkflow, Workflow, WorkflowQuery};
use crate::result::{AppError, JsonResult};
use crate::services::workflows::WorkflowService;

pub async fn list(
    _: UserClaims,
    Query(filter): Query<WorkflowQuery>,
    pool: Data<PoolManager>,
) -> JsonResult<Json<Paginated<Workflow>>> {
    let workflows = block(move || {
        let conn = pool.get()?;
        WorkflowService::new(conn).paginate(filter.into())
    })
    .await??;

    Ok(Json(workflows))
}

pub async fn create(
    _: UserClaims,
    Json(request): Json<NewWorkflow>,
    pool: Data<PoolManager>,
) -> JsonResult<Json<Workflow>> {
    let new_workflow = block(move || {
        let conn = pool.get()?;
        WorkflowService::new(conn).create(request.into())
    })
    .await??;

    Ok(Json(new_workflow))
}

pub async fn find(
    _: UserClaims,
    id: Path<i64>,
    pool: Data<PoolManager>,
) -> JsonResult<Json<Workflow>> {
    let id = id.into_inner();
    let workflow = block(move || {
        let conn = pool.get()?;
        WorkflowService::new(conn).find(id)
    })
    .await??;

    match workflow {
        Some(w) => Ok(Json(w)),
        None => Err(AppError::NotFound {
            entity: "Workflow".to_string(),
            id: id.to_string(),
        }
        .into()),
    }
}

pub async fn update(
    _: UserClaims,
    id: Path<i64>,
    Json(request): Json<UpdateWorkflow>,
    pool: Data<PoolManager>,
) -> JsonResult<Json<Workflow>> {
    let id = id.into_inner();
    let workflow = block(move || {
        let conn = pool.get()?;
        WorkflowService::new(conn).update(id, request)
    })
    .await??;

    Ok(Json(workflow))
}
