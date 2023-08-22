import { Outlet } from "react-router-dom";

function SettingsLayout() {
  return (
    <div className="flex flex-col w-full">
      <Outlet />
    </div>
  );
}

export default SettingsLayout;
