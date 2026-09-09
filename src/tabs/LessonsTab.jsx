import "../styles/components/lessons.css";

const FAKE_LESSONS = [
    { id: "1", label: "Swedish 101", description: "Basics of Swedish grammar and vocabulary.", progress: 0.4 },
    { id: "2", label: "Rust 101", description: "Ownership, borrowing, and your first program.", progress: 0.1 },
    { id: "3", label: "Japanese 101", description: "Hiragana, katakana, and simple phrases.", progress: 0.7 },
];

function monogram(label) {
    return label.trim().charAt(0).toUpperCase();
}

function LessonsTab({ activeLesson, onLessonChange, onAskInChat }) {
    const lesson = FAKE_LESSONS.find((item) => item.id === activeLesson);

    if (lesson) {
        return (
            <section className="lesson-detail">
                <button className="btn back-btn" onClick={() => onLessonChange(null)}>
                    &larr; Back to lessons
                </button>
                <h1>{lesson.label}</h1>
                <p className="lesson-detail-body">
                    Lesson content is coming soon. In the meantime, you can keep exploring this topic in conversation.
                </p>
                {onAskInChat && (
                    <button
                        className="lesson-detail-cta"
                        onClick={() => onAskInChat(`Let's go deeper into ${lesson.label}.`)}
                    >
                        Continue in chat
                    </button>
                )}
            </section>
        );
    }

    return (
        <section className="lessons-grid">
            {FAKE_LESSONS.map((item) => (
                <button key={item.id} className="lesson-card" onClick={() => onLessonChange(item.id)}>
                    <div className="lesson-card-top">
                        <span className="lesson-card-monogram" aria-hidden="true">
                            {monogram(item.label)}
                        </span>
                        <span className="lesson-card-title">{item.label}</span>
                    </div>
                    <span className="lesson-card-description">{item.description}</span>
                    <span className="lesson-card-progress" aria-hidden="true">
                        <span
                            className="lesson-card-progress-fill"
                            style={{ width: `${Math.round(item.progress * 100)}%` }}
                        />
                    </span>
                </button>
            ))}
        </section>
    );
}

export default LessonsTab;
