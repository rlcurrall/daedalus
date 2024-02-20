use actix_web::web::{block, Data, Json, Path, Query};

use crate::{
    database::PoolManager,
    models::{
        common::Paginated,
        workflows::{NewWorkflow, UpdateWorkflow, Workflow, WorkflowQuery},
    },
    result::{AppError, JsonResult},
    services::workflows::WorkflowService,
    UserId,
};

pub async fn list(
    _: UserId,
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
    _: UserId,
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

pub async fn find(_: UserId, id: Path<i64>, pool: Data<PoolManager>) -> JsonResult<Json<Workflow>> {
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
    _: UserId,
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
