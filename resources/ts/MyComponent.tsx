import { useState } from "react";
import { Button } from "./components/button";

export default function MyComponent() {
  const [count, setCount] = useState(0);

  return (
    <div className="flex flex-col gap-4">
      <p>Count: {count}</p>
      <div className="flex gap-2 font-mono">
        <Button className="secondary" onClick={() => setCount(count - 1)}>
          -1
        </Button>
        <Button onClick={() => setCount(count + 1)}>+1</Button>
      </div>
    </div>
  );
}
