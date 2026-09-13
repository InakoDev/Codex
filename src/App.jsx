import "./styles/catppuccin.css";
import "./styles/themes/dark.css";
import "./styles/main.css";

import "./styles/components/welcome.css";

import { getCurrentWindow } from "@tauri-apps/api/window";
import { useState } from "react";
import Sidebar from "./components/Sidebar.jsx";
import Welcome from "./Welcome.jsx";
import LessonsTab from "./tabs/LessonsTab.jsx";
import ChatTab from "./tabs/ChatTab.jsx";
import ProgressTab from "./tabs/ProgressTab.jsx";

function App() {
    const [started, setStarted] = useState(false);
    const [activeTab, setActiveTab] = useState("lessons");
    const [activeLesson, setActiveLesson] = useState(null);
    const [chatQuery, setChatQuery] = useState(null);
    const [chatSessionId, setChatSessionId] = useState(0);

    const appWindow = getCurrentWindow();

    function tabChange(tab) {
        setActiveTab(tab);
    }

    function lessonChange(lesson) {
        setActiveLesson(lesson);
    }

    function startJourney(query) {
        setChatQuery(query);
        setChatSessionId((id) => id + 1);
        setActiveTab("chat");
        setStarted(true);
    }

    function askInChat(prompt) {
        setChatQuery(prompt);
        setChatSessionId((id) => id + 1);
        setActiveTab("chat");
    }

    function renderTab() {
        if (activeTab === "chat") {
            return <ChatTab key={chatSessionId} initialQuery={chatQuery} />;
        }
        if (activeTab === "lessons") {
            return <LessonsTab activeLesson={activeLesson} onLessonChange={lessonChange} onAskInChat={askInChat} />;
        }
        if (activeTab === "progress") {
            return <ProgressTab />;
        }
        return null;
    }

    if (!started) {
        return (
            <main className={"container" /*"container container-welcome"*/}>
                <Welcome onStart={startJourney} onSkip={() => setStarted(true)} />
            </main>
        );
    }

    return (
        <main className="container">
            <Sidebar activeTab={activeTab} onTabChange={tabChange} onLogoClick={() => setStarted(false)} />
            {renderTab()}
        </main>
    );
}

export default App;
