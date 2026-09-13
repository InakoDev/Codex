import { BookIcon, ChatIcon, LessonsIcon, ProgressIcon, SettingsIcon } from "./Icons.jsx";
import "../styles/components/sidebar.css";

const NAVIGATION_ITEMS = [
    { id: "chat", label: "Chat", icon: ChatIcon },
    { id: "lessons", label: "Lessons", icon: LessonsIcon },
    { id: "progress", label: "Progress", icon: ProgressIcon },
];

function Sidebar({ activeTab, onTabChange, onLogoClick }) {
    return (
        <aside className="sidebar">
            <button className="brand" onClick={onLogoClick} type="button">
                <span className="brand-mark" aria-hidden="true">
                    <BookIcon />
                </span>
                <span className="brand-name">Codex</span>
            </button>

            <nav className="nav">
                {NAVIGATION_ITEMS.map((item) => {
                    const Icon = item.icon;
                    const isActive = activeTab === item.id;
                    return (
                        <button
                            key={item.id}
                            className={`btn ${isActive ? "active" : ""}`}
                            onClick={() => onTabChange(item.id)}
                            aria-current={isActive ? "page" : undefined}
                        >
                            <Icon className="btn-icon" />
                            <span>{item.label}</span>
                        </button>
                    );
                })}
            </nav>

            <div className="nav footer">
                <button
                    className={`btn ${activeTab === "settings" ? "active" : ""}`}
                    onClick={() => onTabChange("settings")}
                >
                    <SettingsIcon className="btn-icon" />
                    <span>Settings</span>
                </button>
            </div>
        </aside>
    );
}

export default Sidebar;
