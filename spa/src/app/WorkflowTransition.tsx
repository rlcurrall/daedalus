import { Button } from "@/components/general/button";
import { Text } from "@/components/general/text";
import clsx from "clsx";
import { Handle, NodeProps, Position } from "reactflow";

export default function WorkflowTransition({
  data: transition,
  selected,
}: NodeProps<WorkflowTransition & { dirty: boolean }>) {
  return (
    <>
      <Handle
        type="target"
        position={Position.Top}
        isConnectableStart={false}
      />

      <div
        className={clsx(
          "relative text-white border rounded-full border-zinc-500 bg-zinc-800",
          selected && "ring-1 ring-blue-500",
          transition.dirty && "ring-1 ring-yellow-800"
        )}
      >
        <ActionPanel
          transition={transition}
          selected={selected}
          onAddAction={() => {
            /* noop */
          }}
          onEdit={() => {
            /* noop */
          }}
        />

        <DetailsPanel transition={transition} selected={selected} />

        <Text className="relative font-bold flex gap-2 items-center px-4 py-2">
          <TransitionTypeIcon type={transition.definition.type} />
          {transition.name}
        </Text>
      </div>

      <Handle
        type="source"
        position={Position.Bottom}
        isConnectableEnd={false}
      />
    </>
  );
}

function DetailsPanel({
  transition,
  selected,
}: {
  transition: WorkflowTransition;
  selected: boolean;
}) {
  return (
    <div
      className={clsx(
        "absolute top-1/2 -right-10 w-60 min-h-40 transform -translate-y-[4.5rem] scale-100 bg-zinc-800 transition-all duration-500 rounded flex px-3 py-2 shadow-2xl drop-shadow-2xl",
        selected
          ? "translate-x-full opacity-100"
          : "translate-x-0 opacity-0 scale-0"
      )}
    >
      <dl className="divide-y divide-white/10 w-full">
        <div className="px-4 py-3 sm:grid grid-cols-3 gap-3 sm:px-0">
          <dt className="text-xs font-medium leading-3 text-white">
            <Text>Type</Text>
          </dt>
          <dd className="mt-1 text-sm leading-6 text-gray-400 sm:col-span-2 sm:mt-0">
            <Text>{transition.definition.type}</Text>
          </dd>
        </div>
        <div className="px-4 py-3 sm:grid grid-cols-2 gap-3 sm:px-0">
          <dt className="text-sm font-medium leading-6 text-white">
            <Text>Description</Text>
          </dt>
          <dd className="mt-1 text-sm leading-6 text-gray-400 sm:col-span-2 sm:mt-0">
            <Text>{transition.description ?? "No description"}</Text>
          </dd>
        </div>
      </dl>
    </div>
  );
}

function ActionPanel({
  transition: _,
  selected,
  onAddAction,
  onEdit,
}: {
  transition: WorkflowTransition;
  selected: boolean;
  onAddAction: () => void;
  onEdit: () => void;
}) {
  return (
    <div
      className={clsx(
        "absolute -top-4 left-1/2 -translate-x-1/2 w-max scale-100 bg-zinc-800 transition-all duration-500 transform rounded flex shadow-2xl drop-shadow-2xl",
        selected
          ? "-translate-y-full opacity-100"
          : "translate-y-0 opacity-0 scale-50"
      )}
    >
      <Button
        plain
        square
        onClick={onAddAction}
        className="flex p-2 gap-2 rounded-l"
      >
        <i className="fas fa-bolt" />
        <span>Add Action</span>
      </Button>
      <Button
        plain
        square
        className="flex p-2 gap-2 rounded-r"
        onClick={onEdit}
      >
        <i className="fas fa-pencil-alt" />
        <span>Edit</span>
      </Button>
    </div>
  );
}

function TransitionTypeIcon({
  type,
}: {
  type: WorkflowTransition["definition"]["type"];
}) {
  if (type === "Manual")
    return <i className="fas fa-arrows-split-up-and-left" />;
  if (type === "Automatic")
    return (
      <span className="relative">
        <i className="fas fa-bolt absolute text-[0.5rem] -right-[0.1rem] bottom-[0.1rem] text-amber-600" />
        <i className="fas fa-arrows-split-up-and-left" />
      </span>
    );
  if (type === "Approval")
    return (
      <span className="relative">
        <i className="fas fa-check absolute text-base -right-[0.25rem] -bottom-[0.1rem] text-emerald-600" />
        <i className="fas fa-arrows-split-up-and-left" />
      </span>
    );
  if (type === "VendorConfirmation") return <i className="fas fa-truck" />;
}
