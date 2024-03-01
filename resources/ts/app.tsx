import MyComponent from "./MyComponent";
import ReactDOM from "react-dom/client";
import React from "react";

console.log(`
    ____                 __      __
   / __ \\____ ____  ____/ /___ _/ /_  _______
  / / / / __ \`/ _ \\/ __  / __ \`/ / / / / ___/
 / /_/ / /_/ /  __/ /_/ / /_/ / / /_/ (__  )
/_____/\\__,_/\\___/\\__,_/\\__,_/_/\\__,_/____/

  Welcome to Daedalus!
`);

ReactDOM.createRoot(document.getElementById("root")).render(<MyComponent />);
