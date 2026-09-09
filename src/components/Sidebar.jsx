const NAVIGATION_ITEMS = [
    { id: "chat", label: "Chat" },
    { id: "lessons", label: "Lessons" },
    { id: "progress", label: "Progress" },
];

function Sidebar({ activeTab, onTabChange }) {
    return (
        <aside className="sidebar">
            <span className="title">Codex</span>

            <nav className="sidebar-nav">
                {NAVIGATION_ITEMS.map((item) => (
                    <button
                        key={item.id}
                        className={`btn ${activeTab === item.id ? "active" : ""}`}
                        onClick={() => onTabChange(item.id)}
                    >
                        {item.label}
                    </button>
                ))}
            </nav>
        </aside>
    );
}

export default Sidebar;
