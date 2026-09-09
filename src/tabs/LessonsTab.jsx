import { useState } from "react";

const FAKE_LESSONS = [
    { id: "1", label: "Swedish 101", description: "Basics of Swedish grammar and vocabulary." },
    { id: "2", label: "Rust 101", description: "Ownership, borrowing, and your first program." },
    { id: "3", label: "Japanese 101", description: "Hiragana, katakana, and simple phrases." },
];

function LessonsTab({ activeLesson, onLessonChange }) {
    const lesson = FAKE_LESSONS.find((item) => item.id === activeLesson);

    if (lesson) {
        return (
            <section className="lesson-detail">
                <button className="btn back-btn" onClick={() => onLessonChange(null)}>
                    &larr; Back to Lessons
                </button>
                <h1>{lesson.label}</h1>
                <p>Lesson content coming soon.</p>
            </section>
        );
    }

    return (
        <section className="lessons-grid">
            {FAKE_LESSONS.map((item) => (
                <button key={item.id} className="lesson-card" onClick={() => onLessonChange(item.id)}>
                    <span className="lesson-card-title">{item.label}</span>
                    <span className="lesson-card-description">{item.description}</span>
                </button>
            ))}
        </section>
    );
}

export default LessonsTab;
