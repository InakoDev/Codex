const FAKE_STATS = [
    { id: "journeys", label: "Journeys started", value: "3" },
    { id: "lessons", label: "Lessons completed", value: "12" },
    { id: "streak", label: "Days learning", value: "4" },
];

const FAKE_JOURNEYS = [
    { id: "1", label: "Swedish 101", steps: [true, true, false, false, false] },
    { id: "2", label: "Rust 101", steps: [true, false, false, false, false] },
    { id: "3", label: "Japanese 101", steps: [true, true, true, false, false] },
];

import "../styles/components/progress.css";

function JourneyRow({ journey }) {
    const completed = journey.steps.filter(Boolean).length;
    const percent = Math.round((completed / journey.steps.length) * 100);

    return (
        <li className="journey-row">
            <div className="journey-info">
                <span className="journey-label">{journey.label}</span>
                <span className="journey-percent">{percent}%</span>
            </div>
            <div className="journey-track" aria-hidden="true">
                {journey.steps.map((done, index) => (
                    <span key={index} className={`journey-step ${done ? "done" : ""}`} />
                ))}
            </div>
        </li>
    );
}

function ProgressTab() {
    return (
        <section className="progress-tab">
            <div className="progress-stats">
                {FAKE_STATS.map((stat) => (
                    <div key={stat.id} className="stat-card">
                        <span className="stat-value">{stat.value}</span>
                        <span className="stat-label">{stat.label}</span>
                    </div>
                ))}
            </div>

            <div className="progress-journeys">
                <h2>Your journeys</h2>
                <ul>
                    {FAKE_JOURNEYS.map((journey) => (
                        <JourneyRow key={journey.id} journey={journey} />
                    ))}
                </ul>
            </div>
        </section>
    );
}

export default ProgressTab;
