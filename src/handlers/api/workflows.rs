use actix_identity::Identity;
use actix_web::{
    web::{block, Data, Json, Path, Query},
    Result,
};

use crate::{
    database::PoolManager,
    models::{
        common::Paginated,
        workflows::{NewWorkflow, UpdateWorkflow, Workflow, WorkflowQuery},
    },
    result::AppError,
    services::workflows::WorkflowService,
};

pub async fn list(
    _: Identity,
    Query(filter): Query<WorkflowQuery>,
    pool: Data<PoolManager>,
) -> Result<Json<Paginated<Workflow>>> {
    let workflows = block(move || {
        let conn = pool.get()?;
        WorkflowService::new(conn)
            .paginate(filter.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??;

    Ok(Json(workflows))
}

pub async fn create(
    _: Identity,
    Json(request): Json<NewWorkflow>,
    pool: Data<PoolManager>,
) -> Result<Json<Workflow>> {
    let new_workflow = block(move || {
        let conn = pool.get()?;
        WorkflowService::new(conn)
            .create(request.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??;

    Ok(Json(new_workflow))
}

pub async fn find(_: Identity, id: Path<i64>, pool: Data<PoolManager>) -> Result<Json<Workflow>> {
    let id = id.into_inner();
    let workflow = block(move || {
        let conn = pool.get()?;
        WorkflowService::new(conn)
            .find(id)
            .map_err(|e| Into::<AppError>::into(e))
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
    _: Identity,
    id: Path<i64>,
    Json(request): Json<UpdateWorkflow>,
    pool: Data<PoolManager>,
) -> Result<Json<Workflow>> {
    let id = id.into_inner();
    let workflow = block(move || {
        let conn = pool.get()?;
        WorkflowService::new(conn)
            .update(id, request)
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??;

    Ok(Json(workflow))
}
