import clsx from "clsx";
import { Button } from "../general/button";

export default function ActionPanel({
  selected,
  onAddAction,
  onEdit,
}: {
  selected: boolean;
  onAddAction: () => void;
  onEdit: () => void;
}): JSX.Element {
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
        <i aria-hidden className="fas fa-bolt" />
        <span>Add Action</span>
      </Button>
      <Button
        plain
        square
        className="flex p-2 gap-2 rounded-r"
        onClick={onEdit}
      >
        <i aria-hidden className="fas fa-pencil-alt" />
        <span>Edit</span>
      </Button>
    </div>
  );
}
