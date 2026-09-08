import "./styles/catppuccin.css";
import "./styles/themes/dark.css";
import "./styles/main.css";

import { getCurrentWindow } from "@tauri-apps/api/window";
import { useEffect } from "react";
import Sidebar from "./components/Sidebar.jsx";

function App() {
    const appWindow = getCurrentWindow();

    return (
        <main className="container">
            <Sidebar activeTab="lessons" />
            {/*<h1>Welcome to Tauri + React</h1>*/}
        </main>
    );
}

export default App;
