use crate::{
    database::{DbPool, PooledConnection},
    models::{
        common::Paginated,
        workflows::{NewWorkflow, UpdateWorkflow, Workflow, WorkflowQuery},
    },
    result::{AppError, Result},
};

#[derive(Clone)]
pub struct WorkflowService {
    pool: DbPool,
}

impl WorkflowService {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn find(&self, id: i64) -> Result<Option<Workflow>> {
        let mut conn = self.get_connection()?;

        Ok(Workflow::find(&mut conn, id)?)
    }

    pub fn list(&self, filter: WorkflowQuery) -> Result<Vec<Workflow>> {
        let mut conn = self.get_connection()?;

        Ok(Workflow::list(&mut conn, filter)?)
    }

    pub fn create(&self, workflow: NewWorkflow) -> Result<Workflow> {
        let mut conn = self.get_connection()?;

        Ok(Workflow::create(&mut conn, workflow)?)
    }

    pub fn update(&self, id: i64, workflow: UpdateWorkflow) -> Result<Workflow> {
        let mut conn = self.get_connection()?;

        Ok(Workflow::update(&mut conn, id, workflow)?)
    }

    pub fn count(&self, filter: WorkflowQuery) -> Result<i64> {
        let mut conn = self.get_connection()?;

        Ok(Workflow::count(&mut conn, filter)?)
    }

    pub fn paginate(&self, filter: WorkflowQuery) -> Result<Paginated<Workflow>> {
        let mut conn = self.get_connection()?;

        Ok(Workflow::paginate(&mut conn, filter)?)
    }

    fn get_connection(&self) -> Result<PooledConnection> {
        self.pool.get().map_err(|e| AppError::ServerError {
            cause: e.to_string(),
        })
    }
}
