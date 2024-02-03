use crate::{
    database::PooledConnection,
    models::{
        common::Paginated,
        workflows::{NewWorkflow, UpdateWorkflow, Workflow, WorkflowQuery},
    },
    result::Result,
};

pub struct WorkflowService {
    conn: PooledConnection,
}

impl WorkflowService {
    pub fn new(conn: PooledConnection) -> Self {
        Self { conn }
    }

    pub fn find(&mut self, id: i64) -> Result<Option<Workflow>> {
        Ok(Workflow::find(&mut self.conn, id)?)
    }

    pub fn list(&mut self, filter: WorkflowQuery) -> Result<Vec<Workflow>> {
        Ok(Workflow::list(&mut self.conn, filter)?)
    }

    pub fn create(&mut self, workflow: NewWorkflow) -> Result<Workflow> {
        Ok(Workflow::create(&mut self.conn, workflow)?)
    }

    pub fn update(&mut self, id: i64, workflow: UpdateWorkflow) -> Result<Workflow> {
        Ok(Workflow::update(&mut self.conn, id, workflow)?)
    }

    pub fn count(&mut self, filter: WorkflowQuery) -> Result<i64> {
        Ok(Workflow::count(&mut self.conn, filter)?)
    }

    pub fn paginate(&mut self, filter: WorkflowQuery) -> Result<Paginated<Workflow>> {
        Ok(Workflow::paginate(&mut self.conn, filter)?)
    }
}
