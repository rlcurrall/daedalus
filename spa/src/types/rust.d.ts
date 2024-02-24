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
  created_at: Date;
  updated_at: Date;
  deleted_at?: Date;
}

interface WorkflowDefinition {
  initial_state: string;
  states: Array<WorkflowState>;
}

interface WorkflowState {
  id: string;
  name: string;
  description?: string;
  entry_actions: Array<WorkflowAction>;
  exit_actions: Array<WorkflowAction>;
  transitions: Array<WorkflowTransition>;
  position: WorkflowPosition;
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
  position: WorkflowPosition;
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
  options: Array<TransitionOption>;
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
  label: string;
  target_state_id: string;
  data: Array<TransitionOptionData>;
}

type TransitionOptionData =
  | TransitionOptionData__Comment
  | TransitionOptionData__Date
  | TransitionOptionData__VendorId;

type TransitionOptionData__Comment = {
  type: "Comment";
};
type TransitionOptionData__Date = {
  type: "Date";
  label: string;
};
type TransitionOptionData__VendorId = {
  type: "VendorId";
  label: string;
};
