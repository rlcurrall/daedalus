import clsx from "clsx";
import { Handle, NodeProps, Position } from "reactflow";
import { PencilSquareIcon } from "@heroicons/react/24/solid";
import { BoltIcon } from "@heroicons/react/24/outline";
import {
  Dialog,
  DialogActions,
  DialogBody,
  DialogDescription,
  DialogTitle,
} from "@/components/general/dialog";
import { Text } from "@/components/general/text";
import { useEffect, useState } from "react";
import { Button } from "@/components/general/button";
import { Input } from "@/components/general/input";
import {
  ErrorMessage,
  Field,
  FieldGroup,
  Fieldset,
  Label,
} from "@/components/general/fieldset";
import { Select } from "@/components/general/select";

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
          "relative bg-zinc-900 text-white border rounded-md border-zinc-500 pb-2",
          selected && "ring-1 ring-blue-500",
          state.dirty && "ring-1 ring-yellow-800"
        )}
      >
        <div
          className={clsx(
            "absolute -top-4 left-1/2 -translate-x-1/2 w-max scale-100 bg-zinc-800 transition-all duration-500 transform rounded flex",
            selected
              ? "-translate-y-full opacity-100"
              : "translate-y-0 opacity-0"
          )}
        >
          <Button
            plain
            square
            onClick={() => {
              setSelectedActionTiming(undefined);
              setSelectedAction(undefined);
              setShowActionModal(true);
            }}
            className="flex p-2 gap-2 rounded-l"
          >
            <BoltIcon className="w-6 h-6" />
            <span>Add Action</span>
          </Button>
          <Button
            plain
            square
            className="flex p-2 gap-2 rounded-r"
            onClick={() => setShowStateModal(true)}
          >
            <PencilSquareIcon className="w-6 h-6" />
            <span>Edit</span>
          </Button>
        </div>

        <Text className="relative font-bold rounded-t-md px-4 py-2 border-b border-zinc-500 bg-zinc-800">
          {state.name}
        </Text>

        {state.description && (
          <Text className="px-4 py-2">{state.description}</Text>
        )}

        <div className="px-4 py-2 min-h-16 min-w-48 flex gap-4">
          {state.entry_actions.length > 0 && (
            <div className="flex flex-col grow gap-2">
              <Text className="px-1">Entry Actions</Text>
              {state.entry_actions.map((action) => (
                <Button
                  key={action.id}
                  className="flex items-center justify-between"
                  color="dark"
                  onClick={() => {
                    setSelectedActionTiming("entry");
                    setSelectedAction(action);
                    setShowActionModal(true);
                  }}
                >
                  {action.name}
                  <PencilSquareIcon className="w-4 h-4" />
                </Button>
              ))}
            </div>
          )}
          {state.exit_actions.length > 0 && (
            <div className="flex flex-col grow gap-2">
              <Text className="px-1">Exit Actions</Text>
              {state.exit_actions.map((action) => (
                <Button
                  key={action.id}
                  className="flex items-center justify-between"
                  color="dark"
                  onClick={() => {
                    setSelectedActionTiming("exit");
                    setSelectedAction(action);
                    setShowActionModal(true);
                  }}
                >
                  {action.name}
                  <PencilSquareIcon className="w-4 h-4" />
                </Button>
              ))}
            </div>
          )}
        </div>

        <Handle type="source" position={Position.Bottom} />
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
        onChange={(name, description) => {
          state.name = name;
          state.description = description;
          state.dirty = true;
          setShowStateModal(false);
        }}
      />
    </>
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
    action: WorkflowAction
  ) => ActionChangeResult;
  onUpdate: (
    timing: "entry" | "exit",
    action: WorkflowAction
  ) => ActionChangeResult;
  onRemove: (action: WorkflowAction) => ActionChangeResult;
}) {
  const [timing, setTiming] = useState<"entry" | "exit">(oldTiming ?? "entry");
  const [name, setActionName] = useState(action?.name ?? "");
  const [type, setActionType] = useState<WorkflowActionType>(
    action?.definition.type ?? "AutoAssign"
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
        : undefined
    );
    setUserId(
      action?.definition && "user_id" in action?.definition
        ? action?.definition.user_id
        : undefined
    );
    setEmail(
      action?.definition && "email" in action?.definition
        ? action?.definition.email
        : undefined
    );
    setErrors({});
  }, [action]);

  return (
    <Dialog size="xl" open={open} onClose={onClose}>
      <DialogTitle>{action ? "Update Action" : "Add Action"}</DialogTitle>

      <DialogDescription>
        {action ? "Update" : "Add"} an action to the {timing} of this state.
      </DialogDescription>

      <DialogBody>
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
      </DialogBody>

      <DialogActions>
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
            <div className="grow" />
          </>
        )}
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
      </DialogActions>
    </Dialog>
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
  onChange: (name: string, description: string | undefined) => void;
}) {
  const [name, setName] = useState(state.name);
  const [description, setDescription] = useState(state.description);

  return (
    <Dialog size="xl" open={open} onClose={onClose}>
      <DialogTitle>Edit State</DialogTitle>
      <DialogDescription>
        Edit the state&apos;s name and description.
      </DialogDescription>
      <DialogBody>
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
        </Fieldset>
      </DialogBody>
      <DialogActions>
        <Button color="dark" onClick={onClose}>
          Cancel
        </Button>
        <Button color="blue" onClick={() => onChange(name, description)}>
          Save
        </Button>
      </DialogActions>
    </Dialog>
  );
}
