import { Text } from "@/components/general/text";
import ActionPanel from "@/components/workflow/ActionPanel";
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
          "relative rounded-full border border-zinc-500 bg-zinc-800 text-white",
          selected && "ring-1 ring-blue-500",
          transition.dirty && "ring-1 ring-yellow-800",
        )}
      >
        <ActionPanel
          selected={selected}
          onAddAction={() => {
            /* noop */
          }}
          onEdit={() => {
            /* noop */
          }}
        />

        <DetailsPanel transition={transition} selected={selected} />

        <Text className="relative flex items-center gap-2 px-4 py-2 font-bold">
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
        "absolute -right-10 top-1/2 flex min-h-40 w-60 -translate-y-[4.5rem] scale-100 transform rounded bg-zinc-800 px-3 py-2 shadow-2xl drop-shadow-2xl transition-all duration-500",
        selected
          ? "translate-x-full opacity-100"
          : "translate-x-0 scale-0 opacity-0",
      )}
    >
      <dl className="w-full divide-y divide-white/10">
        <div className="grid-cols-3 gap-3 px-4 py-3 sm:grid sm:px-0">
          <dt className="text-xs font-medium leading-3 text-white">
            <Text>Type</Text>
          </dt>
          <dd className="mt-1 text-sm leading-6 text-gray-400 sm:col-span-2 sm:mt-0">
            <Text>{transition.definition.type}</Text>
          </dd>
        </div>
        <div className="grid-cols-2 gap-3 px-4 py-3 sm:grid sm:px-0">
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

function TransitionTypeIcon({
  type,
}: {
  type: WorkflowTransition["definition"]["type"];
}) {
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
    return <i aria-hidden className="fas fa-truck" />;
}
