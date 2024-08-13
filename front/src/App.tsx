import './App.css'

import { Router } from "@solidjs/router";

const routes = [{
  path: "/",
  component: import("./routes/index.tsx"),
}]

function App() {
  return (
    <Router>
      {routes}
    </Router>
  )
}

export default App
