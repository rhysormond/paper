// async import of entire app
import("./src")
  .catch(e => console.error("Error importing `index.js`:", e));
