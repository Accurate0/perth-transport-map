import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import {
  createBrowserRouter,
  Navigate,
  RouterProvider,
} from "react-router-dom";
import MapRoute from "./routes/Map";

const router = createBrowserRouter([
  {
    path: "/map",
    element: <MapRoute />,
  },
  {
    path: "*",
    element: <Navigate to="/map" />,
  },
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
);
