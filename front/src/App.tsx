import { Route, Routes } from "react-router-dom";
import RootLayout from "./components/RootLayout";
import SettingsLayout from "./components/SettingsLayout";
import SettingsIndex from "./pages/settings/index";

function App() {
  return (
    <>
      <Routes>
        <Route path="/" element={<RootLayout />}>
          <Route index element={<div>Home</div>} />
          <Route path="dashboard" element={<div>Dashboard</div>} />
          <Route path="settings" element={<SettingsLayout />}>
            <Route index element={<SettingsIndex />} />
          </Route>

          <Route path="*" element={<div>Nothing path</div>} />
        </Route>
      </Routes>
    </>
  );
}

export default App;
