import clsx from "clsx";
import { Button } from "../general/button";

export default function TopPanel({
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
        "absolute -top-4 left-1/2 flex w-max -translate-x-1/2 scale-100 transform rounded bg-zinc-800 shadow-2xl drop-shadow-2xl transition-all duration-500",
        selected
          ? "-translate-y-full opacity-100"
          : "translate-y-0 scale-50 opacity-0",
      )}
    >
      {onAddAction && (
        <Button
          plain
          onClick={onAddAction}
          className={clsx("flex gap-2 rounded-l p-2", onEdit && "rounded-r")}
        >
          <i aria-hidden className="fas fa-bolt" />
          <span>Add Action</span>
        </Button>
      )}
      {onEdit && (
        <Button
          plain
          className={clsx("flex gap-2 p-2", onAddAction && "rounded-r")}
          onClick={onEdit}
        >
          <i aria-hidden className="fas fa-pencil-alt" />
          <span>Edit</span>
        </Button>
      )}
    </div>
  );
}
