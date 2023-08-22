import { Outlet } from "react-router-dom";
import Sidebar from "./Sidebar";

function RootLayout() {
  return (
    <div className="flex min-w-screen">
      <Sidebar />
      <Outlet />
    </div>
  );
}

export default RootLayout;
