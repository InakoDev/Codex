// import ProgressRing from "./ProgressRing";
// import { domains, categories, suggestedModels } from "../data/mockData";

const NAVIGATION_ITEMS = [
    { id: "lessons", label: "Lessons" },
    { id: "chat", label: "Esex (ft. Thoq)" },
    { id: "progress", label: "Progress" },
];

function Sidebar({ activeDomain, onDomainChange, activeCategory, onCategoryChange, activeTab, onTabChange }) {
    // const domainCategories = categories[activeDomain];
    // const domainAccent = domains.find((d) => d.id === activeDomain)?.accent;

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

            {/*<div className="sidebar-section-label">Categories</div>*/}
        </aside>
    );
}

export default Sidebar;
