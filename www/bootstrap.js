// async import of entire app
import("./index.js")
  .catch(e => console.error("Error importing `index.js`:", e));
