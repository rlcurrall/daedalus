import React, { useState } from 'react';

export default function MyComponent() {
  const [count, setCount] = useState(0);
  return <div>
    <p>Count: {count}</p>
    <div role="group">
      <button onClick={() => setCount(count + 1)}>+1</button>
      <button className="secondary" onClick={() => setCount(count - 1)}>-1</button>
    </div>
  </div>;
}