use actix_web::{
    get, post,
    web::{block, Data, Json, Path, Query},
};

use crate::{
    models::{
        common::Paginated,
        workflows::{NewWorkflow, UpdateWorkflow, Workflow, WorkflowQuery},
    },
    result::AppError,
    services::workflows::WorkflowService,
};

#[get("/")]
pub async fn list(
    Query(filter): Query<WorkflowQuery>,
    workflow_service: Data<WorkflowService>,
) -> actix_web::Result<Json<Paginated<Workflow>>> {
    let workflows = block(move || {
        workflow_service
            .paginate(filter.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??;

    Ok(Json(workflows))
}

#[post("/")]
pub async fn create(
    Json(request): Json<NewWorkflow>,
    workflow_service: Data<WorkflowService>,
) -> actix_web::Result<Json<Workflow>> {
    let new_workflow = block(move || {
        workflow_service
            .create(request.into())
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??;

    Ok(Json(new_workflow))
}

#[get("/{id}")]
pub async fn get(
    id: Path<i64>,
    workflow_service: Data<WorkflowService>,
) -> actix_web::Result<Json<Workflow>> {
    let id = id.into_inner();
    let workflow = block(move || {
        workflow_service
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

#[post("/{id}")]
pub async fn update(
    id: Path<i64>,
    Json(request): Json<UpdateWorkflow>,
    workflow_service: Data<WorkflowService>,
) -> actix_web::Result<Json<Workflow>> {
    let id = id.into_inner();
    let workflow = block(move || {
        workflow_service
            .update(id, request)
            .map_err(|e| Into::<AppError>::into(e))
    })
    .await??;

    Ok(Json(workflow))
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(list);
    cfg.service(create);
    cfg.service(get);
    cfg.service(update);
}
