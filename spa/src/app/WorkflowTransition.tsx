import { Button } from "@/components/general/button";
import {
  Description,
  Field,
  FieldGroup,
  Fieldset,
  Label,
  Legend,
} from "@/components/general/fieldset";
import { Input } from "@/components/general/input";
import { Select } from "@/components/general/select";
import { SlideOver } from "@/components/general/slide-over";
import { Switch } from "@/components/general/switch";
import { Strong, Text } from "@/components/general/text";
import { Textarea } from "@/components/general/textarea";
import clsx from "clsx";
import { useEffect, useState } from "react";
import {
  Handle,
  MarkerType,
  NodeProps,
  Position,
  useEdges,
  useNodes,
} from "reactflow";
import { useEditorContext } from "./WorkflowEditor";
import { Ids, WorkflowEdgeData } from "./utils";

export default function WorkflowTransition({
  data: transition,
  selected,
}: NodeProps<WorkflowTransition & { dirty: boolean }>) {
  const { edges, setEdges, nodes, setNodes } = useEditorContext();

  const [modalState, setModalState] = useState<"edit" | "view" | "closed">(
    "closed",
  );

  return (
    <div>
      <Handle
        type="target"
        position={Position.Top}
        isConnectableStart={false}
      />

      <div
        className={clsx(
          "relative rounded-full border border-zinc-500 bg-zinc-800 text-white",
          selected && "ring-1 ring-blue-500",
          transition.dirty && "ring-1 ring-yellow-800",
        )}
      >
        <ActionPanel
          open={selected}
          onEdit={() => setModalState("view")}
          onRemove={() => setNodes(nodes.filter((n) => n.id !== transition.id))}
        />

        <Text className="relative flex items-center gap-2 px-4 py-2 font-bold">
          <TransitionTypeIcon type={transition.definition.type} />
          {transition.name}
        </Text>
      </div>

      <DetailsPanel
        transition={transition}
        open={modalState === "view"}
        onClose={() => setModalState("closed")}
        onEdit={() => setModalState("edit")}
      />
      <EditPanel
        transition={transition}
        open={modalState === "edit"}
        onClose={() => setModalState("closed")}
        onCancel={() => setModalState("view")}
        onChange={(updated) => {
          transition.name = updated.name;
          transition.description = updated.description;
          transition.definition = updated.definition;
          transition.dirty = true;

          // deep copy of edges, just to make sure we don't run into an issue with react
          let edgesClone = JSON.parse(
            JSON.stringify(
              edges.filter((edge) => edge.source !== transition.id),
            ),
          ) as typeof edges;

          if (updated.definition.type === "Approval")
            edgesClone.push(
              {
                id: Ids.transitionApproveId(updated.id),
                source: updated.id,
                target: updated.definition.approval_option.target_state_id,
                type: "smoothstep",
                label: updated.definition.approval_option.label,
                markerEnd: { type: MarkerType.ArrowClosed },
                style: { strokeWidth: 2 },
              },
              {
                id: Ids.transitionRejectId(updated.id),
                source: updated.id,
                target: updated.definition.rejection_option.target_state_id,
                type: "smoothstep",
                label: updated.definition.rejection_option.label,
                markerEnd: { type: MarkerType.ArrowClosed },
                style: { strokeWidth: 2 },
              },
            );

          if (
            updated.definition.type === "Automatic" ||
            updated.definition.type === "VendorConfirmation"
          )
            edgesClone.push({
              id: Ids.transitionTargetId(updated.id),
              source: updated.id,
              target: updated.definition.target_state_id,
              type: "smoothstep",
              markerEnd: { type: MarkerType.ArrowClosed },
              style: { strokeWidth: 2 },
            });

          if (updated.definition.type === "Manual")
            updated.definition.options.forEach((option) => {
              edgesClone.push({
                id: Ids.transitionOptionId(updated.id, option.id),
                source: updated.id,
                target: option.target_state_id,
                type: "smoothstep",
                label: option.label,
                markerEnd: { type: MarkerType.ArrowClosed },
                style: { strokeWidth: 2 },
              });
            });

          setEdges(edgesClone);
          setModalState("view");
        }}
      />

      <Handle
        type="source"
        position={Position.Bottom}
        isConnectableEnd={false}
      />
    </div>
  );
}

function DetailsPanel({
  transition,
  open,
  onClose,
  onEdit,
}: {
  transition: WorkflowTransition;
  open: boolean;
  onClose(open: boolean): void;
  onEdit(): void;
}) {
  return (
    <SlideOver open={open} onClose={onClose}>
      <SlideOver.Title>
        <Strong>Transition Details</Strong>
      </SlideOver.Title>

      <SlideOver.Panel className="flex h-full flex-col gap-4">
        <div className="space-y-2 text-sm leading-6 sm:mt-0">
          <h1 className="text-lg">
            <Strong>{transition.name}</Strong>
          </h1>
          <Text>
            {transition.description ??
              "No description... here is a bunch of text to see what it looks like with more data..."}
          </Text>
        </div>

        <dl className="w-full divide-y divide-zinc-700/20">
          <div className="grid-cols-2 gap-3 py-4 sm:grid sm:px-0">
            <dt className="text-sm font-medium leading-3">
              <Text>
                <Strong>Type</Strong>
              </Text>
            </dt>
            <dd className="mt-1 text-sm leading-6 sm:mt-0">
              <Text>{transition.definition.type}</Text>
            </dd>
          </div>
          {"approver_id" in transition.definition && (
            <div className="grid-cols-2 gap-3 py-4 sm:grid sm:px-0">
              <dt className="text-sm font-medium leading-6">
                <Text>
                  <Strong>Approver</Strong>
                </Text>
              </dt>
              <dd className="mt-1 text-sm leading-6 sm:mt-0">
                <Text>{transition.definition.approver_id}</Text>
              </dd>
            </div>
          )}

          {"options" in transition.definition && (
            <div className="gap-3 py-4 sm:grid sm:px-0">
              <dt className="text-sm font-medium leading-6">
                <Strong>Target States</Strong>
              </dt>
              <dd className="mt-1 grid gap-2 text-sm leading-6 sm:mt-0">
                {transition.definition.options.map((option) => (
                  <TransitionOption key={option.id} option={option} />
                ))}
              </dd>
            </div>
          )}

          {"approval_option" in transition.definition &&
            "rejection_option" in transition.definition && (
              <div className="gap-3 py-4 sm:grid sm:px-0">
                <dt className="text-sm font-medium leading-6">
                  <Strong>Approval Options</Strong>
                </dt>
                <dd className="mt-1 grid gap-4 text-sm leading-6 sm:mt-0">
                  <TransitionOption
                    option={transition.definition.approval_option}
                  />
                  <TransitionOption
                    option={transition.definition.rejection_option}
                  />
                </dd>
              </div>
            )}
        </dl>

        <div className="grow" />

        <div className="flex justify-end">
          <Button color="blue" onClick={onEdit}>
            <i aria-hidden className="fas fa-edit" />
            <span>Edit</span>
          </Button>
        </div>
      </SlideOver.Panel>
    </SlideOver>
  );
}

function TransitionOption({ option }: { option: TransitionOption }) {
  const nodes = useNodes<WorkflowState | WorkflowTransition>();
  const targetState = nodes.find(
    (n) => n.id === option.target_state_id && n.type === "state",
  )?.data.name;

  return (
    <div className="w-full overflow-hidden rounded-lg border border-zinc-400 bg-zinc-300 dark:border-zinc-700 dark:bg-zinc-800">
      <Text className="border-b border-zinc-400 px-4 py-2 dark:border-zinc-700">
        <Strong>{option.label}</Strong>
      </Text>

      <div className="bg-white px-4 py-2 dark:bg-zinc-900/70">
        <Text className="flex gap-2">
          <Strong>Target</Strong>
          <span>{targetState}</span>
        </Text>

        <div className={clsx("space-y-2", option.data.length && "mt-4")}>
          {option.comment_required && (
            <Text className="ml-4 flex items-center gap-2">
              <i aria-hidden className="fas fa-comment" />
              Comment Required
            </Text>
          )}
          {option.data.map((data) => (
            <Text key={data.id} className="ml-4 flex items-center gap-2">
              {data.type === "UserId" && (
                <>
                  <i aria-hidden className="fas fa-user" /> {data.label}
                </>
              )}
              {data.type === "Date" && (
                <>
                  <i aria-hidden className="fas fa-calendar" /> {data.label}
                </>
              )}
              {data.type === "VendorId" && (
                <>
                  <i aria-hidden className="fas fa-truck" /> {data.label}
                </>
              )}
            </Text>
          ))}
        </div>
      </div>
    </div>
  );
}

function EditPanel({
  open,
  transition,
  onClose,
  onCancel,
  onChange,
}: {
  open: boolean;
  transition: WorkflowTransition;
  onClose(): void;
  onCancel(): void;
  onChange(transition: WorkflowTransition): void;
}) {
  const sourceStates = useEdges<WorkflowEdgeData>()
    .filter((edge) => edge.target === transition.id)
    .map((edge) => edge.source);
  const availableStates = useNodes<WorkflowState | WorkflowTransition>()
    .filter((n) => n.type === "state" && !sourceStates.includes(n.id))
    .map(({ id, data: { name } }) => ({ id, name }));

  const [name, setName] = useState(transition.name);
  const [description, setDescription] = useState(transition.description ?? "");
  const [type, setType] = useState(transition.definition.type);
  const [approver_id, setApproverId] = useState(
    "approver_id" in transition.definition
      ? transition.definition.approver_id
      : "",
  );
  const [target_state_id, setTargetStateId] = useState(
    "target_state_id" in transition.definition
      ? transition.definition.target_state_id
      : "",
  );
  const [options, setOptions] = useState(
    "options" in transition.definition ? transition.definition.options : [],
  );
  const [approval_option, setApprovalOption] = useState(
    "approval_option" in transition.definition
      ? transition.definition.approval_option
      : {
          id: crypto.randomUUID(),
          label: "",
          target_state_id: "",
          comment_required: false,
          data: [],
        },
  );
  const [rejection_option, setRejectionOption] = useState(
    "rejection_option" in transition.definition
      ? transition.definition.rejection_option
      : {
          id: crypto.randomUUID(),
          label: "",
          target_state_id: "",
          comment_required: false,
          data: [],
        },
  );

  const submit = (data: FormData) => {
    let definition: TransitionDefinition;

    switch (type) {
      case "Automatic":
        definition = { type, target_state_id };
        break;
      case "Approval":
        definition = {
          type,
          approver_id: typeof approver_id === "number" ? approver_id : 0,
          approval_option,
          rejection_option,
        };
        break;
      case "Manual":
        definition = { type, options };
        break;
      case "VendorConfirmation":
        definition = { type, target_state_id };
        break;
    }

    onChange({ id: transition.id, name, description, definition });
  };

  const reset = () => {
    setName(transition.name);
    setDescription(transition.description ?? "");
    setType(transition.definition.type);
    setApproverId(
      "approver_id" in transition.definition
        ? transition.definition.approver_id
        : "",
    );
    setTargetStateId(
      "target_state_id" in transition.definition
        ? transition.definition.target_state_id
        : "",
    );
    setOptions(
      "options" in transition.definition ? transition.definition.options : [],
    );
    setApprovalOption(
      "approval_option" in transition.definition
        ? transition.definition.approval_option
        : {
            id: crypto.randomUUID(),
            label: "",
            target_state_id: "",
            comment_required: false,
            data: [],
          },
    );
    setRejectionOption(
      "rejection_option" in transition.definition
        ? transition.definition.rejection_option
        : {
            id: crypto.randomUUID(),
            label: "",
            target_state_id: "",
            comment_required: false,
            data: [],
          },
    );
  };

  useEffect(() => {
    if (open) reset();
  }, [open]);

  return (
    <SlideOver open={open} onClose={onClose}>
      <SlideOver.Title>
        <Strong>Edit Transition</Strong>
      </SlideOver.Title>

      <SlideOver.Panel className="h-full">
        <form
          action={submit}
          onReset={reset}
          className="flex h-full flex-col gap-4"
        >
          <Fieldset>
            <Legend>Transition Details</Legend>
            <Description>
              Edit the transition details below. Fields marked with an asterisk
              (*) are required.
            </Description>
            <FieldGroup>
              <Field>
                <Label>
                  Name <span className="text-rose-400">*</span>
                </Label>
                <Input
                  required
                  name="name"
                  defaultValue={name}
                  onChange={(e) => setName(e.target.value)}
                  placeholder="Transition Name"
                />
              </Field>
            </FieldGroup>
            <FieldGroup>
              <Field>
                <Label>Description</Label>
                <Textarea
                  name="description"
                  defaultValue={description}
                  onChange={(e) => setDescription(e.target.value)}
                  placeholder="Transition Description"
                />
              </Field>
            </FieldGroup>
            <FieldGroup>
              <Field>
                <Label>
                  Type <span className="text-rose-400">*</span>
                </Label>
                <Description>
                  Select the type of transition to define the behavior.
                </Description>
                <Select
                  required
                  name="type"
                  defaultValue={type}
                  onChange={(e) =>
                    setType(e.target.value as TransitionDefinition["type"])
                  }
                >
                  <option disabled value="" hidden>
                    Select Type
                  </option>
                  <option value="Manual">Manual</option>
                  <option value="Automatic">Automatic</option>
                  <option value="Approval">Approval</option>
                  <option value="VendorConfirmation">
                    Vendor Confirmation
                  </option>
                </Select>
              </Field>
            </FieldGroup>
            {type === "Approval" && (
              <FieldGroup>
                <Field>
                  <Label>
                    Approver <span className="text-rose-400">*</span>
                  </Label>
                  <Description>
                    Select the user who will approve the transition.
                  </Description>
                  <Select
                    required
                    name="approver_id"
                    defaultValue={approver_id === 0 ? "" : approver_id}
                    onChange={(e) => setApproverId(parseInt(e.target.value))}
                  >
                    <option disabled value="" hidden>
                      Select Approver
                    </option>
                    {/* Todo, fetch these from an API */}
                    <option value="1">User 1</option>
                    <option value="2">User 2</option>
                    <option value="3">User 3</option>
                  </Select>
                </Field>
              </FieldGroup>
            )}

            {(type === "Automatic" || type === "VendorConfirmation") && (
              <FieldGroup>
                <Field>
                  <Label>
                    Target <span className="text-rose-400">*</span>
                  </Label>
                  <Description>
                    Select the state to transition to when the condition is met.
                  </Description>
                  <Select
                    required
                    name="target_state_id"
                    defaultValue={target_state_id}
                    onChange={(e) => setTargetStateId(e.target.value)}
                  >
                    <option disabled value="" hidden>
                      Select Target State
                    </option>
                    {availableStates.map((state) => (
                      <option key={state.id} value={state.id}>
                        {state.name}
                      </option>
                    ))}
                  </Select>
                </Field>
              </FieldGroup>
            )}

            {type === "Manual" && (
              <>
                <div className="mb-4 mt-8">
                  <Text>
                    <Strong>Options</Strong>
                  </Text>
                  <Text>
                    Define the options available to the user when transitioning
                    between states.
                  </Text>
                </div>

                <div className="grid gap-4">
                  {options.map((option, index) => (
                    <OptionField
                      key={option.id}
                      option={option}
                      title={`Option #${index + 1}`}
                      availableStates={availableStates}
                      onRemove={(id) =>
                        setOptions(options.filter((o) => o.id !== id))
                      }
                      onChange={(o) =>
                        setOptions(
                          options.map((op) => (op.id === o.id ? o : op)),
                        )
                      }
                    />
                  ))}
                </div>

                <FieldGroup>
                  <Button
                    type="button"
                    className="w-full"
                    onClick={() =>
                      setOptions([
                        ...options,
                        {
                          id: crypto.randomUUID(),
                          label: "",
                          target_state_id: "",
                          comment_required: false,
                          data: [],
                        },
                      ])
                    }
                  >
                    <i aria-hidden className="fas fa-plus" />
                    Add Option
                  </Button>
                </FieldGroup>
              </>
            )}

            {type === "Approval" && (
              <>
                <div className="mb-4 mt-8">
                  <Text>
                    <Strong>Approval Options</Strong>
                  </Text>
                  <Text>
                    Define the options available to the approver when reviewing
                    the transition.
                  </Text>
                </div>

                <div className="grid gap-4">
                  <OptionField
                    key={approval_option.id}
                    title="Approval"
                    option={approval_option}
                    availableStates={availableStates}
                    onChange={setApprovalOption}
                  />

                  <OptionField
                    key={rejection_option.id}
                    title="Rejection"
                    option={rejection_option}
                    availableStates={availableStates}
                    onChange={setRejectionOption}
                  />
                </div>
              </>
            )}
          </Fieldset>

          <div className="grow" />

          <div className="flex gap-4 py-4">
            <Button color="rose" type="button" onClick={() => onCancel()}>
              Cancel
            </Button>
            <div className="grow" />
            <Button type="reset">Reset</Button>
            <Button color="emerald" type="submit">
              Save
            </Button>
          </div>
        </form>
      </SlideOver.Panel>
    </SlideOver>
  );
}

function OptionField({
  option,
  title,
  availableStates,
  onRemove,
  onChange,
}: {
  option: TransitionOption;
  title: string;
  availableStates: { id: string; name: string }[];
  onRemove?(id: string): void;
  onChange(option: TransitionOption): void;
}): JSX.Element {
  return (
    <div className="flex flex-col gap-4 rounded border border-zinc-700 bg-zinc-800 p-3 pt-2">
      <FieldGroup>
        <Legend className=" flex items-center justify-between">
          <span>{title}</span>
          {onRemove && (
            <Button plain type="button" onClick={() => onRemove(option.id)}>
              <i aria-hidden className="fas fa-trash text-rose-500" />
              <span className="sr-only">Remove Option</span>
            </Button>
          )}
        </Legend>
        <Field>
          <Label>
            Label <span className="text-rose-400">*</span>
          </Label>
          <Input
            required
            name={`options-[${option.id}].label`}
            defaultValue={option.label}
            onChange={(e) => onChange({ ...option, label: e.target.value })}
            placeholder="Option Label"
          />
        </Field>

        <Field>
          <Label>
            Target State <span className="text-rose-400">*</span>
          </Label>
          <Select
            required
            name={`options-[${option.id}].target_state_id`}
            defaultValue={
              availableStates.find((o) => o.id === option.target_state_id)
                ? option.target_state_id
                : ""
            }
            onChange={(e) =>
              onChange({ ...option, target_state_id: e.target.value })
            }
          >
            <option disabled value="" hidden>
              Select Target State
            </option>
            {availableStates.map((state) => (
              <option key={state.id} value={state.id}>
                {state.name}
              </option>
            ))}
          </Select>
        </Field>

        <Field className="space-x-2">
          <Label>
            Comment Required <span className="text-rose-400">*</span>
          </Label>
          <Switch
            color="blue"
            name={`options-[${option.id}].comment_required`}
            defaultChecked={option.comment_required}
            onChange={(comment_required) =>
              onChange({ ...option, comment_required })
            }
          />
        </Field>
      </FieldGroup>

      {option.data.map((data, i) => (
        <OptionDataField
          key={data.id}
          data={data}
          onRemove={(id) => {
            const data = option.data.filter((d) => d.id !== id);
            onChange({ ...option, data });
          }}
          onChange={(data) => {
            const updated = option.data.map((d) =>
              d.id === data.id ? { ...d, ...data } : d,
            );
            onChange({ ...option, data: updated });
          }}
        />
      ))}

      <div>
        <Button
          type="button"
          onClick={() =>
            onChange({
              ...option,
              data: [
                ...option.data,
                {
                  id: crypto.randomUUID(),
                  type: "Date",
                  label: "",
                },
              ],
            })
          }
        >
          <i aria-hidden className="fas fa-plus" />
          Add Field
        </Button>
      </div>
    </div>
  );
}

function OptionDataField({
  data,
  onRemove,
  onChange,
}: {
  data: TransitionOptionData;
  onRemove(id: string): void;
  onChange(data: TransitionOptionData): void;
}): JSX.Element {
  return (
    <>
      <hr className="border-zinc-300 dark:border-zinc-700" />
      <FieldGroup>
        <Field className="flex items-center justify-between gap-2">
          <Select
            name={`option.data-[${data.id}].type`}
            defaultValue={data.type}
            onChange={(e) => {
              const type = e.target.value as TransitionOptionData["type"];
              onChange({ ...data, type });
            }}
          >
            <option disabled value="" hidden>
              Select Field Type
            </option>
            <option value="UserId">User</option>
            <option value="Date">Date</option>
            <option value="VendorId">Vendor</option>
          </Select>
          <Input
            name={`option.data-[${data.id}].label`}
            defaultValue={data.label}
            onChange={(e) => onChange({ ...data, label: e.target.value })}
            placeholder={
              {
                UserId: "User Label",
                Date: "Date Label",
                VendorId: "Vendor Label",
              }[data.type]
            }
          />
          <Button plain type="button" onClick={() => onRemove(data.id)}>
            <i aria-hidden className="fas fa-trash text-rose-500" />
            <span className="sr-only">Remove Option</span>
          </Button>
        </Field>
      </FieldGroup>
    </>
  );
}

function TransitionTypeIcon({ type }: { type: TransitionDefinition["type"] }) {
  if (type === "Manual")
    return <i aria-hidden className="fas fa-arrows-split-up-and-left" />;
  if (type === "Automatic")
    return (
      <span className="relative">
        <i
          aria-hidden
          className="fas fa-bolt absolute -right-[0.1rem] bottom-[0.1rem] text-[0.5rem] text-amber-600"
        />
        <i aria-hidden className="fas fa-arrows-split-up-and-left" />
      </span>
    );
  if (type === "Approval")
    return (
      <span className="relative">
        <i
          aria-hidden
          className="fas fa-check absolute -bottom-[0.1rem] -right-[0.25rem] text-base text-emerald-600"
        />
        <i aria-hidden className="fas fa-arrows-split-up-and-left" />
      </span>
    );
  if (type === "VendorConfirmation")
    return (
      <span className="relative">
        <i
          aria-hidden
          className="fas fa-check absolute -bottom-[0.1rem] -right-[0.25rem] text-base text-emerald-600"
        />
        <i aria-hidden className="fas fa-truck" />
      </span>
    );
}

function ActionPanel({
  open,
  onEdit,
  onRemove,
}: {
  open: boolean;
  onEdit(): void;
  onRemove(): void;
}): JSX.Element {
  return (
    <div
      className={clsx(
        "absolute -top-4 left-1/2 flex w-max -translate-x-1/2 scale-100 transform gap-2 transition-all duration-500",
        open
          ? "-translate-y-full opacity-100"
          : "translate-y-0 scale-50 opacity-0",
      )}
    >
      <Button color="blue" onClick={onEdit}>
        <i aria-hidden className="fas fa-circle-info" />
        <span>Details</span>
      </Button>
      <Button color="rose" onClick={onRemove}>
        <i aria-hidden className="fas fa-trash" />
        <span>Remove</span>
      </Button>
    </div>
  );
}
