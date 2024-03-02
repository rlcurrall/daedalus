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

const root = document.getElementById("root");
if (root) ReactDOM.createRoot(root).render(<MyComponent />);
else console.error("No root element found");
