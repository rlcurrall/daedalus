import clsx from "clsx";
import { Handle, NodeProps, Position } from "reactflow";

export default function WorkflowTransition({
  data: transition,
  selected,
}: NodeProps<WorkflowTransition>) {
  return (
    <div
      className={clsx(
        "text-white bg-zinc-900 px-4 py-2 rounded border-zinc-500 border",
        selected && "ring-1 ring-blue-500"
      )}
    >
      <Handle type="target" position={Position.Top} />
      {transition.name}
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
}
