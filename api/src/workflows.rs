use std::collections::HashMap;
use std::io::Write;

use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::PgValue;
use diesel::prelude::*;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Jsonb;
use serde::{Deserialize, Serialize};
use tsync::tsync;
use uuid::Uuid;

use crate::database::{schema::workflows, DbConnection, DB};
use crate::defaults::{default_bool, default_i64};
use crate::result::AppError;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[tsync]
pub struct WorkflowPosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Deserialize, Insertable, Serialize)]
#[diesel(table_name = workflows)]
#[tsync]
pub struct NewWorkflow {
    pub tenant_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub definition: WorkflowDefinition,
}

#[derive(AsChangeset, Clone, Deserialize, Serialize)]
#[diesel(table_name = workflows)]
#[tsync]
pub struct UpdateWorkflow {
    pub name: Option<String>,
    pub description: Option<String>,
    pub definition: Option<WorkflowDefinition>,
}

#[derive(Clone, Deserialize, Serialize)]
#[tsync]
pub struct WorkflowQuery {
    pub tenant_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(default = "default_bool::<true>")]
    pub active: bool,
    #[serde(default = "default_i64::<1>")]
    pub page: i64,
    #[serde(default = "default_i64::<10>")]
    pub per_page: i64,
}

impl Workflow {
    pub fn create(
        conn: &mut DbConnection,
        new_workflow: NewWorkflow,
    ) -> Result<Workflow, AppError> {
        let res = diesel::insert_into(workflows::table)
            .values(new_workflow)
            .get_result(conn)?;

        Ok(res)
    }

    pub fn update(
        conn: &mut DbConnection,
        id: i64,
        update_workflow: UpdateWorkflow,
    ) -> Result<Workflow, AppError> {
        let res = diesel::update(workflows::table)
            .filter(workflows::id.eq(id))
            .set(update_workflow)
            .get_result(conn)?;

        Ok(res)
    }

    pub fn find(conn: &mut DbConnection, id: i64) -> Result<Option<Workflow>, AppError> {
        let res = workflows::table
            .select(Workflow::as_select())
            .filter(workflows::id.eq(id))
            .get_result(conn)
            .optional()?;

        Ok(res)
    }

    pub fn count(conn: &mut DbConnection, query: WorkflowQuery) -> Result<i64, AppError> {
        let mut q = workflows::table
            .select(diesel::dsl::count(workflows::id))
            .into_boxed();

        if let Some(tenant_id) = query.tenant_id {
            q = q.filter(workflows::tenant_id.eq(tenant_id));
        }

        if let Some(name) = query.name {
            q = q.filter(workflows::name.eq(name));
        }

        if let Some(description) = query.description {
            q = q.filter(workflows::description.eq(description));
        }

        let res = q.get_result(conn)?;

        Ok(res)
    }

    pub fn list(conn: &mut DbConnection, query: WorkflowQuery) -> Result<Vec<Workflow>, AppError> {
        let mut q = workflows::table.select(Workflow::as_select()).into_boxed();

        if let Some(tenant_id) = query.tenant_id {
            q = q.filter(workflows::tenant_id.eq(tenant_id));
        }

        if let Some(name) = query.name {
            q = q.filter(workflows::name.eq(name));
        }

        if let Some(description) = query.description {
            q = q.filter(workflows::description.eq(description));
        }

        let res = q
            .offset((query.page - 1) * query.per_page)
            .limit(query.per_page)
            .get_results(conn)?;

        Ok(res)
    }
}

#[derive(Deserialize, Queryable, Identifiable, Selectable, Serialize)]
#[tsync]
#[diesel(table_name = workflows)]
pub struct Workflow {
    pub id: i64,
    pub tenant_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub definition: WorkflowDefinition,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl FromSql<Jsonb, DB> for WorkflowDefinition {
    fn from_sql(bytes: PgValue) -> diesel::deserialize::Result<Self> {
        let bytes = bytes.as_bytes();

        if bytes[0] != 1 {
            return Err("Unsupported JSONB version".into());
        }
        serde_json::from_slice(&bytes[1..]).map_err(Into::into)
    }
}

impl ToSql<Jsonb, DB> for WorkflowDefinition {
    fn to_sql(&self, out: &mut Output<DB>) -> serialize::Result {
        out.write_all(&[1])?;
        serde_json::to_writer(out, self)
            .map(|_| IsNull::No)
            .map_err(Into::into)
    }
}

#[derive(AsExpression, Clone, Debug, Deserialize, FromSqlRow, Serialize)]
#[tsync]
#[diesel(sql_type = Jsonb)]
pub struct WorkflowDefinition {
    pub initial_state: Uuid,
    pub states: Vec<WorkflowState>,
    pub metadata: WorkflowMetadata,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[tsync]
pub struct WorkflowMetadata {
    pub positions: HashMap<Uuid, WorkflowPosition>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[tsync]
pub struct WorkflowState {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_end_state: bool,
    pub entry_actions: Vec<WorkflowAction>,
    pub exit_actions: Vec<WorkflowAction>,
    pub transitions: Vec<WorkflowTransition>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[tsync]
pub struct WorkflowAction {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub definition: ActionDefinition,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[tsync]
#[serde(tag = "type")]
pub enum ActionDefinition {
    AutoAssign,
    AssignTo {
        user_id: i64,
    },
    Email {
        template_id: i64,
        email: String,
    },
    Notify {
        template_id: i64,
        target: NotifyTarget,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[tsync]
#[serde(tag = "type")]
pub enum NotifyTarget {
    Creator,
    User { id: i64 },
    Vendor,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[tsync]
pub struct WorkflowTransition {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub definition: TransitionDefinition,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[tsync]
#[serde(tag = "type")]
pub enum TransitionDefinition {
    Automatic {
        target_state_id: Uuid,
    },
    Approval {
        approver_id: i64,
        approval_option: TransitionOption,
        rejection_option: TransitionOption,
    },
    Manual {
        options: Vec<TransitionOption>,
    },
    VendorConfirmation {
        target_state_id: Uuid,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[tsync]
pub struct TransitionOption {
    pub id: Uuid,
    pub label: String,
    pub target_state_id: Uuid,
    pub comment_required: bool,
    pub data: Vec<TransitionOptionData>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[tsync]
#[serde(tag = "type")]
pub enum TransitionOptionData {
    Date { id: Uuid, label: String },
    UserId { id: Uuid, label: String },
    VendorId { id: Uuid, label: String },
}
