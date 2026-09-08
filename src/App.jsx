import "./App.css";

import { getCurrentWindow } from "@tauri-apps/api/window";
import { useEffect } from "react";

function App() {
    const appWindow = getCurrentWindow();

    useEffect(() => {
        const interval = setInterval(() => {
            console.log(`width: ${window.innerWidth}px, height: ${window.innerHeight}px`);
        }, 500);

        return () => clearInterval(interval);
    }, []);

    return (
        <main className="container">
            <h1>Welcome to Tauri + React</h1>
        </main>
    );
}

export default App;
