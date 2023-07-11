import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import {
  createBrowserRouter,
  Navigate,
  RouterProvider,
} from "react-router-dom";
import MapRoute from "./routes/Map";
import { CssBaseline } from "@mui/material";

const router = createBrowserRouter([
  {
    path: "/map",
    element: <MapRoute />,
  },
  {
    path: "*",
    element: <Navigate replace to="/map" />,
  },
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <CssBaseline />
    <RouterProvider router={router} />
  </React.StrictMode>
);
