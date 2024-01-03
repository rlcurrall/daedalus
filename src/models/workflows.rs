use diesel::sql_types::Uuid;

pub struct Workflow {
    pub id: Uuid,
    pub tenant_id: i32,
    pub name: String,
    pub description: String,
    pub initial_state_id: Option<Uuid>,
}

pub struct WorkflowState {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub name: String,
    pub description: String,
}

pub struct WorkflowAction {
    pub id: Uuid,
    pub state_id: Uuid,
    pub name: String,
    pub description: String,
    pub timing: ActionTrigger,
    pub configuration: ActionConfiguration,
}

pub struct WorkflowTrigger {
    pub id: Uuid,
    pub state_id: Uuid,
    pub name: String,
    pub description: String,
    pub configuration: TriggerConfiguration,
}

pub enum ActionTrigger {
    Entry,
    Exit,
}

pub enum ActionConfiguration {
    AutoAssign,
    AssignTo { user_id: i64 },
    Email { template_id: i64, email: String },
    NotifyCreator { template_id: i64 },
    NotifyManager { template_id: i64, user_id: i64 },
    NotifyVendor { template_id: i64 },
}

pub enum TriggerConfiguration {
    Automatic {
        target_state_id: Uuid,
    },
    Approval {
        approver_id: i64,
        target_state_id: Uuid,
    },
    Manual {
        options: Vec<ManualTriggerOption>,
    },
    VendorConfirmation {
        target_state_id: Uuid,
    },
}

pub struct ManualTriggerOption {
    pub label: String,
    pub target_state_id: Uuid,
    pub data: Vec<TriggerOptionData>,
}

pub enum TriggerOptionData {
    Comment,
    Date { label: String },
    VendorId { label: String },
}
