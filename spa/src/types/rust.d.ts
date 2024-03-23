/* This file is generated and managed by tsync */

interface Paginated<T> {
  total: number;
  page: number;
  per_page: number;
  data: Array<T>;
}

interface Tenant {
  id: number;
  name: string;
  created_at: Date;
  updated_at: Date;
  deleted_at?: Date;
}

interface TenantQuery {
  name?: string;
  active: boolean;
  page: number;
  page_size: number;
}

interface CreateTenant {
  name: string;
}

interface UpdateTenant {
  name?: string;
}

interface CreateUser {
  tenant_id: number;
  email: string;
  password: string;
}

interface User {
  id: number;
  tenant_id: number;
  email: string;
  password: string;
  created_at: Date;
  updated_at: Date;
  deleted_at?: Date;
}

interface UserQuery {
  tenant_id?: number;
  email?: string;
  active: boolean;
  page: number;
  page_size: number;
}

interface WorkflowPosition {
  x: number;
  y: number;
}

interface NewWorkflow {
  tenant_id: number;
  name: string;
  description?: string;
  definition: WorkflowDefinition;
}

interface UpdateWorkflow {
  name?: string;
  description?: string;
  definition?: WorkflowDefinition;
}

interface WorkflowQuery {
  tenant_id?: number;
  name?: string;
  description?: string;
  active: boolean;
  page: number;
  per_page: number;
}

interface Workflow {
  id: number;
  tenant_id: number;
  name: string;
  description?: string;
  definition: WorkflowDefinition;
  editor_metadata: WorkflowEditorMetadata;
  created_at: Date;
  updated_at: Date;
  deleted_at?: Date;
}

interface WorkflowEditorMetadata {
  positions: Record<string, WorkflowPosition>;
}

interface WorkflowDefinition {
  initial_state: string;
  states: Array<WorkflowState>;
}

interface WorkflowState {
  id: string;
  name: string;
  description?: string;
  is_end_state: boolean;
  entry_actions: Array<WorkflowAction>;
  exit_actions: Array<WorkflowAction>;
  transitions: Array<WorkflowTransition>;
}

interface WorkflowAction {
  id: string;
  name: string;
  description?: string;
  definition: ActionDefinition;
}

type ActionDefinition =
  | ActionDefinition__AutoAssign
  | ActionDefinition__AssignTo
  | ActionDefinition__Email
  | ActionDefinition__Notify;

type ActionDefinition__AutoAssign = {
  type: "AutoAssign";
};
type ActionDefinition__AssignTo = {
  type: "AssignTo";
  user_id: number;
};
type ActionDefinition__Email = {
  type: "Email";
  template_id: number;
  email: string;
};
type ActionDefinition__Notify = {
  type: "Notify";
  template_id: number;
  target: NotifyTarget;
};

type NotifyTarget =
  | NotifyTarget__Creator
  | NotifyTarget__User
  | NotifyTarget__Vendor;

type NotifyTarget__Creator = {
  type: "Creator";
};
type NotifyTarget__User = {
  type: "User";
  id: number;
};
type NotifyTarget__Vendor = {
  type: "Vendor";
};

interface WorkflowTransition {
  id: string;
  name: string;
  description?: string;
  definition: TransitionDefinition;
}

type TransitionDefinition =
  | TransitionDefinition__Automatic
  | TransitionDefinition__Approval
  | TransitionDefinition__Manual
  | TransitionDefinition__VendorConfirmation;

type TransitionDefinition__Automatic = {
  type: "Automatic";
  target_state_id: string;
};
type TransitionDefinition__Approval = {
  type: "Approval";
  approver_id: number;
  approval_option: TransitionOption;
  rejection_option: TransitionOption;
};
type TransitionDefinition__Manual = {
  type: "Manual";
  options: Array<TransitionOption>;
};
type TransitionDefinition__VendorConfirmation = {
  type: "VendorConfirmation";
  target_state_id: string;
};

interface TransitionOption {
  id: string;
  label: string;
  target_state_id: string;
  comment_required: boolean;
  data: Array<TransitionOptionData>;
}

type TransitionOptionData =
  | TransitionOptionData__Date
  | TransitionOptionData__UserId
  | TransitionOptionData__VendorId;

type TransitionOptionData__Date = {
  id: string;
  type: "Date";
  label: string;
};
type TransitionOptionData__UserId = {
  id: string;
  type: "UserId";
  label: string;
};
type TransitionOptionData__VendorId = {
  id: string;
  type: "VendorId";
  label: string;
};
