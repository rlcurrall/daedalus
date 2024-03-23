import { Button } from "@/components/general/button";
import {
  ErrorMessage,
  Field,
  FieldGroup,
  Fieldset,
  Label,
} from "@/components/general/fieldset";
import { Input } from "@/components/general/input";
import { Select } from "@/components/general/select";
import { SlideOver } from "@/components/general/slide-over";
import { Switch } from "@/components/general/switch";
import { Strong, Text } from "@/components/general/text";
import clsx from "clsx";
import { useEffect, useState } from "react";
import { Handle, NodeProps, Position } from "reactflow";

export default function WorkflowState({
  data: state,
  selected,
}: NodeProps<WorkflowState & { dirty: boolean }>) {
  const [showStateModal, setShowStateModal] = useState(false);
  const [showActionModal, setShowActionModal] = useState(false);
  const [selectedAction, setSelectedAction] = useState<
    WorkflowAction | undefined
  >(undefined);
  const [selectedActionTiming, setSelectedActionTiming] = useState<
    "entry" | "exit" | undefined
  >(undefined);

  return (
    <>
      <Handle type="target" position={Position.Top} />

      <div
        className={clsx(
          "relative rounded-md border border-zinc-500 bg-zinc-900 pb-2 text-white",
          selected && "ring-1 ring-blue-500",
          state.dirty && "ring-1 ring-yellow-800",
        )}
      >
        <TopPanel
          selected={selected}
          onAddAction={() => {
            setSelectedActionTiming(undefined);
            setSelectedAction(undefined);
            setShowActionModal(true);
          }}
          onEdit={() => setShowStateModal(true)}
        />

        <Text className="relative rounded-t-md border-b border-zinc-500 bg-zinc-800 px-4 py-2 font-bold">
          {state.name}
        </Text>

        {state.description && (
          <Text className="px-4 py-2">{state.description}</Text>
        )}

        <div className="flex min-h-16 min-w-48 gap-4 px-4 py-2">
          {state.entry_actions.length > 0 && (
            <ActionList
              actions={state.entry_actions}
              timing="entry"
              onEdit={(action) => {
                setSelectedActionTiming("entry");
                setSelectedAction(action);
                setShowActionModal(true);
              }}
            />
          )}
          {state.exit_actions.length > 0 && (
            <ActionList
              actions={state.exit_actions}
              timing="exit"
              onEdit={(action) => {
                setSelectedActionTiming("exit");
                setSelectedAction(action);
                setShowActionModal(true);
              }}
            />
          )}
        </div>

        {!state.is_end_state && (
          <Handle type="source" position={Position.Bottom} />
        )}
      </div>

      <AddOrEditActionModal
        action={selectedAction}
        timing={selectedActionTiming}
        open={showActionModal}
        onClose={() => {
          setShowActionModal(false);
          setSelectedAction(undefined);
          setSelectedActionTiming(undefined);
        }}
        onAdd={(timing, action) => {
          if (timing === "entry") state.entry_actions.push(action);
          else state.exit_actions.push(action);
          state.dirty = true;
          return { success: true };
        }}
        onUpdate={(timing, action) => {
          const original =
            timing === "entry"
              ? state.entry_actions.find((a) => a.id === action.id)
              : state.exit_actions.find((a) => a.id === action.id);
          if (!original)
            return { success: false, errors: { general: "Action not found" } };
          original.name = action.name;
          original.definition = action.definition;
          state.dirty = true;
          return { success: true };
        }}
        onRemove={(action) => {
          let index = state.entry_actions.findIndex((a) => a.id === action.id);
          if (index >= 0) {
            state.entry_actions.splice(index, 1);
            state.dirty = true;
            return { success: true };
          }

          index = state.exit_actions.findIndex((a) => a.id === action.id);
          if (index >= 0) {
            state.exit_actions.splice(index, 1);
            state.dirty = true;
            return { success: true };
          }

          return {
            success: false,
            errors: { general: "Action not found" },
          };
        }}
      />
      <EditStateModal
        open={showStateModal}
        onClose={() => setShowStateModal(false)}
        state={state}
        onChange={(name, description, isEndState) => {
          state.name = name;
          state.description = description;
          state.is_end_state = isEndState;
          state.dirty = true;
          setShowStateModal(false);
        }}
      />
    </>
  );
}

function TopPanel({
  selected,
  onAddAction,
  onEdit,
}: {
  selected: boolean;
  onAddAction?: () => void;
  onEdit?: () => void;
}): JSX.Element {
  return (
    <div
      className={clsx(
        "absolute -top-4 left-1/2 flex w-max -translate-x-1/2 scale-100 transform gap-4 rounded transition-all duration-500",
        selected
          ? "-translate-y-full opacity-100"
          : "translate-y-0 scale-50 opacity-0",
      )}
    >
      {onAddAction && (
        <Button
          color="orange"
          onClick={onAddAction}
          className={clsx("flex gap-2 p-2")}
        >
          <i aria-hidden className="fas fa-bolt" />
          <span>Add Action</span>
        </Button>
      )}
      {onEdit && (
        <Button
          color="blue"
          className={clsx("flex gap-2 p-2")}
          onClick={onEdit}
        >
          <i aria-hidden className="fas fa-pencil-alt" />
          <span>Edit</span>
        </Button>
      )}
    </div>
  );
}

function ActionList({
  actions,
  timing,
  onEdit,
}: {
  actions: WorkflowAction[];
  timing: "entry" | "exit";
  onEdit: (action: WorkflowAction) => void;
}) {
  return (
    <div className="flex grow flex-col gap-2">
      <Text className="px-1">
        {timing === "entry" ? "Entry" : "Exit"} Actions
      </Text>
      {actions.map((action) => (
        <Button
          key={action.id}
          className="flex items-center justify-between"
          color="dark"
          onClick={() => onEdit(action)}
        >
          {action.name}
          <i aria-hidden className="fas fa-pencil-alt" />
        </Button>
      ))}
    </div>
  );
}

type ActionChangeResult =
  | { success: true }
  | { success: false; errors: Record<string, string | undefined> };
type ActionDefinitionResult =
  | { success: true; definition: WorkflowAction["definition"] }
  | { success: false; errors: Record<string, string | undefined> };
type WorkflowActionType = WorkflowAction["definition"]["type"];

function AddOrEditActionModal({
  timing: oldTiming,
  action,
  open,
  onClose,
  onAdd,
  onUpdate,
  onRemove,
}: {
  timing?: "entry" | "exit";
  action?: WorkflowAction;
  open: boolean;
  onClose: () => void;
  onAdd: (
    timing: "entry" | "exit",
    action: WorkflowAction,
  ) => ActionChangeResult;
  onUpdate: (
    timing: "entry" | "exit",
    action: WorkflowAction,
  ) => ActionChangeResult;
  onRemove: (action: WorkflowAction) => ActionChangeResult;
}) {
  const [timing, setTiming] = useState<"entry" | "exit">(oldTiming ?? "entry");
  const [name, setActionName] = useState(action?.name ?? "");
  const [type, setActionType] = useState<WorkflowActionType>(
    action?.definition.type ?? "AutoAssign",
  );
  const [template_id, setTemplateId] = useState<number | undefined>();
  const [user_id, setUserId] = useState<number | undefined>();
  const [email, setEmail] = useState<string | undefined>();
  const [errors, setErrors] = useState<Record<string, string | undefined>>({});
  const [notify_target_type, setNotifyTargetType] =
    useState<NotifyTarget["type"]>("Creator");
  const [notify_target_id, setNotifyTargetId] = useState<number | undefined>();

  const handleClose = () => {
    reset();
    onClose();
  };

  const reset = () => {
    setActionName("");
    setActionType("AutoAssign");
    setTemplateId(undefined);
    setUserId(undefined);
    setEmail(undefined);
    setNotifyTargetType("Creator");
    setNotifyTargetId(undefined);
    setErrors({});
  };

  const getDefinition = (): ActionDefinitionResult => {
    switch (type) {
      case "AutoAssign":
        return { success: true, definition: { type } };
      case "AssignTo":
        if (!user_id)
          return { success: false, errors: { user_id: "User ID is required" } };
        return { success: true, definition: { type, user_id } };
      case "Email":
        const eErrors: Record<string, string | undefined> = {};
        if (!email) eErrors["email"] = "Email is required";
        if (!template_id) eErrors["template_id"] = "Template ID is required";
        if (Object.keys(eErrors).length > 0)
          return { success: false, errors: eErrors };
        return {
          success: true,
          definition: { type, email: email!, template_id: template_id! },
        };
      case "Notify":
        const nErrors: Record<string, string | undefined> = {};
        if (!template_id) nErrors["template_id"] = "Template ID is required";
        if (!notify_target_type)
          nErrors["target_type"] = "Target type is required";
        if (notify_target_type === "User" && notify_target_id === undefined)
          nErrors["target_id"] = "Target ID is required";
        if (Object.keys(nErrors).length > 0)
          return { success: false, errors: nErrors };
        return {
          success: true,
          definition: {
            type,
            template_id: template_id!,
            target: {
              type: notify_target_type!,
              id: notify_target_id!,
            },
          },
        };
    }
  };

  useEffect(() => {
    setTiming(oldTiming ?? "entry");
  }, [oldTiming]);

  useEffect(() => {
    setActionName(action?.name ?? "");
    setActionType(action?.definition.type ?? "AutoAssign");
    setTemplateId(
      action?.definition && "template_id" in action?.definition
        ? action?.definition.template_id
        : undefined,
    );
    setUserId(
      action?.definition && "user_id" in action?.definition
        ? action?.definition.user_id
        : undefined,
    );
    setEmail(
      action?.definition && "email" in action?.definition
        ? action?.definition.email
        : undefined,
    );
    setErrors({});
  }, [action]);

  return (
    <SlideOver open={open} onClose={onClose}>
      <SlideOver.Title>
        <Text>
          <Strong className="text-xl">
            {action ? "Update Action" : "Add Action"}
          </Strong>
        </Text>
      </SlideOver.Title>

      <SlideOver.Panel className="flex h-full flex-col gap-4">
        <Text>
          {action ? "Update" : "Add"} an action to the {timing} of this state.
        </Text>

        <Fieldset>
          {!!errors.general && <ErrorMessage>{errors.general}</ErrorMessage>}

          <FieldGroup>
            <Field>
              <Label>Action Name</Label>
              <Input
                name="action_name"
                value={name}
                onChange={({ target: { value } }) => setActionName(value)}
                invalid={!!errors.name}
              />
              {!!errors.name && <ErrorMessage>{errors.name}</ErrorMessage>}
            </Field>
            <Field>
              <Label>Timing</Label>
              <Select
                value={timing}
                onChange={({ target: { value } }) =>
                  setTiming(value as "entry" | "exit")
                }
                invalid={!!errors.timing}
              >
                <option value="entry">Entry</option>
                <option value="exit">Exit</option>
              </Select>
              {!!errors.timing && <ErrorMessage>{errors.timing}</ErrorMessage>}
            </Field>
            <Field>
              <Label>Action Type</Label>
              <Select
                value={type}
                onChange={({ target: { value } }) =>
                  setActionType(value as WorkflowAction["definition"]["type"])
                }
                invalid={!!errors.type}
              >
                <option value="AutoAssign">Auto-assign</option>
                <option value="AssignTo">Assign To</option>
                <option value="Email">Email</option>
                <option value="Notify">Notify</option>
              </Select>
              {!!errors.type && <ErrorMessage>{errors.type}</ErrorMessage>}
            </Field>
            {
              {
                AutoAssign: <></>,
                AssignTo: (
                  <AssignToFields
                    userId={user_id}
                    setUserId={setUserId}
                    errors={errors}
                  />
                ),
                Email: (
                  <EmailFields
                    email={email}
                    setEmail={setEmail}
                    errors={errors}
                  />
                ),
                Notify: (
                  <NotifyFields
                    templateId={template_id}
                    setTemplateId={setTemplateId}
                    targetType={notify_target_type}
                    setTargetType={setNotifyTargetType}
                    targetId={notify_target_id}
                    setTargetId={setNotifyTargetId}
                    errors={errors}
                  />
                ),
              }[type]
            }
          </FieldGroup>
        </Fieldset>

        <div className="grow" />

        <div className="mt-4 flex gap-4">
          {!!action && (
            <>
              <Button
                color="red"
                onClick={() => {
                  if (!onRemove) return;
                  const result = onRemove(action);
                  if (result.success) handleClose();
                  else setErrors(result.errors);
                }}
              >
                Remove
              </Button>
            </>
          )}
          <div className="grow" />
          <Button color="dark" onClick={onClose}>
            Cancel
          </Button>
          <Button
            color="blue"
            onClick={() => {
              setErrors({});
              const definitionResult = getDefinition();
              if (!definitionResult.success) {
                setErrors(definitionResult.errors);
                return;
              }

              const result = !action
                ? onAdd(timing, {
                    id: crypto.randomUUID(),
                    name,
                    definition: definitionResult.definition,
                  })
                : onUpdate(timing, {
                    id: action.id,
                    name,
                    definition: definitionResult.definition,
                  });

              if (result.success) handleClose();
              else setErrors(result.errors);
            }}
          >
            Save
          </Button>
        </div>
      </SlideOver.Panel>
    </SlideOver>
  );
}

function AssignToFields({
  userId,
  setUserId,
  errors,
}: {
  userId: number | undefined;
  setUserId: (id: number | undefined) => void;
  errors: Record<string, string | undefined>;
}) {
  return (
    <Field>
      <Label>User ID</Label>
      <Input
        name="user_id"
        type="number"
        value={userId}
        onChange={({ target: { value } }) =>
          setUserId(value ? parseInt(value) : undefined)
        }
        invalid={!!errors.user_id}
      />
      {!!errors.user_id && <ErrorMessage>{errors.user_id}</ErrorMessage>}
    </Field>
  );
}

function EmailFields({
  email,
  setEmail,
  errors,
}: {
  email: string | undefined;
  setEmail: (email: string | undefined) => void;
  errors: Record<string, string | undefined>;
}) {
  return (
    <Field>
      <Label>Email</Label>
      <Input
        name="email"
        type="email"
        value={email}
        onChange={({ target: { value } }) => setEmail(value)}
        invalid={!!errors.email}
      />
      {!!errors.email && <ErrorMessage>{errors.email}</ErrorMessage>}
    </Field>
  );
}

function NotifyFields({
  templateId,
  setTemplateId,
  targetType,
  setTargetType,
  targetId,
  setTargetId,
  errors,
}: {
  templateId: number | undefined;
  setTemplateId: (id: number | undefined) => void;
  targetType: NotifyTarget["type"];
  setTargetType: (type: NotifyTarget["type"]) => void;
  targetId: number | undefined;
  setTargetId: (id: number | undefined) => void;
  errors: Record<string, string | undefined>;
}) {
  return (
    <>
      <Field>
        <Label>Template ID</Label>
        <Input
          name="template_id"
          type="number"
          value={templateId}
          onChange={({ target: { value } }) =>
            setTemplateId(value ? parseInt(value) : undefined)
          }
          invalid={!!errors.template_id}
        />
        {!!errors.template_id && (
          <ErrorMessage>{errors.template_id}</ErrorMessage>
        )}
      </Field>
      <Field>
        <Label>Target Type</Label>
        <Select
          value={targetType}
          onChange={({ target: { value } }) =>
            setTargetType(value as NotifyTarget["type"])
          }
          invalid={!!errors.target_type}
        >
          <option value="Creator">Creator</option>
          <option value="User">User</option>
          <option value="Vendor">Vendor</option>
        </Select>
        {!!errors.target_type && (
          <ErrorMessage>{errors.target_type}</ErrorMessage>
        )}
      </Field>
      {targetType === "User" && (
        <Field>
          <Label>Target ID</Label>
          <Input
            name="target_id"
            type="number"
            value={targetId}
            onChange={({ target: { value } }) =>
              setTargetId(value ? parseInt(value) : undefined)
            }
            invalid={!!errors.target_id}
          />
          {!!errors.target_id && (
            <ErrorMessage>{errors.target_id}</ErrorMessage>
          )}
        </Field>
      )}
    </>
  );
}

function EditStateModal({
  open,
  onClose,
  state,
  onChange,
}: {
  open: boolean;
  onClose: () => void;
  state: WorkflowState;
  onChange: (
    name: string,
    description: string | undefined,
    isEndState: boolean,
  ) => void;
}) {
  const [name, setName] = useState(state.name);
  const [description, setDescription] = useState(state.description);
  const [isEndState, setIsEndState] = useState(state.is_end_state);

  return (
    <SlideOver open={open} onClose={onClose}>
      <SlideOver.Title>
        <Text>
          <Strong className="text-xl">Edit State</Strong>
        </Text>
      </SlideOver.Title>

      <SlideOver.Panel className="flex h-full flex-col gap-4">
        <Text>Edit the state&apos;s name and description.</Text>

        <Fieldset className="space-y-4">
          <Field>
            <Label>State Name</Label>
            <Input
              name="state_name"
              value={name}
              onChange={(e) => setName(e.target.value)}
            />
          </Field>

          <Field>
            <Label>Description</Label>
            <Input
              name="state_description"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
            />
          </Field>

          <Field className="space-x-4">
            <Label>Is End State</Label>
            <Switch
              color="blue"
              checked={isEndState}
              onChange={(checked) => setIsEndState(checked)}
            />
          </Field>
        </Fieldset>

        <div className="grow" />

        <div className="flex gap-4">
          <div className="grow" />
          <Button color="dark" onClick={onClose}>
            Cancel
          </Button>
          <Button
            color="blue"
            onClick={() => onChange(name, description, isEndState)}
          >
            Save
          </Button>
        </div>
      </SlideOver.Panel>
    </SlideOver>
  );
}
