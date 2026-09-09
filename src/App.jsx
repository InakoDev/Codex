import "./styles/catppuccin.css";
import "./styles/themes/dark.css";
import "./styles/main.css";

import { getCurrentWindow } from "@tauri-apps/api/window";
import { useEffect, useState } from "react";
import Sidebar from "./components/Sidebar.jsx";
import LessonsTab from "./tabs/LessonsTab.jsx";

function App() {
    const [activeTab, setActiveTab] = useState("lessons");
    const [activeLesson, setActiveLesson] = useState(null);

    const appWindow = getCurrentWindow();

    function tabChange(tab) {
        setActiveTab(tab);
    }

    function lessonChange(lesson) {
        setActiveLesson(lesson);
    }

    function renderTab() {
        if (activeTab === "chat") {
            return <h1>Meow</h1>;
        }
        if (activeTab === "lessons") {
            return <LessonsTab activeLesson={activeLesson} onLessonChange={lessonChange} />;
        }
        if (activeTab === "progress") {
            // return <h1>Progress</h1>;
        }
        return null;
    }

    return (
        <main className="container">
            <Sidebar activeTab={activeTab} onTabChange={tabChange} />
            {renderTab()}
        </main>
    );
}

export default App;
