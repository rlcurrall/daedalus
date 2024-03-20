use actix_web::web::{block, Data, Json, Path, Query};

use crate::database::PoolManager;
use crate::middleware::bearer::UserClaims;
use crate::result::{AppError, JsonResult};
use crate::workflows::{NewWorkflow, UpdateWorkflow, Workflow, WorkflowQuery};

use super::Paginated;

pub async fn list(
    _: UserClaims,
    Query(filter): Query<WorkflowQuery>,
    pool: Data<PoolManager>,
) -> JsonResult<Json<Paginated<Workflow>>> {
    let workflows = block(move || -> Result<_, AppError> {
        let mut conn = pool.get()?;
        let total = Workflow::count(&mut conn, filter.clone())?;
        let data = Workflow::list(&mut conn, filter.clone())?;

        Ok(Paginated {
            total,
            page: filter.page,
            per_page: filter.per_page,
            data,
        })
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
        let mut conn = pool.get()?;
        Workflow::create(&mut conn, request.into())
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
        let mut conn = pool.get()?;
        Workflow::find(&mut conn, id)
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
        let mut conn = pool.get()?;
        Workflow::update(&mut conn, id, request)
    })
    .await??;

    Ok(Json(workflow))
}
