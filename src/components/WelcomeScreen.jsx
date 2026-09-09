import { useState } from "react";
import { ArrowIcon } from "./Icons.jsx";

const PROMPT_EXAMPLES = ["How does a neural network actually learn?", "Explain ownership and borrowing in Rust"];

function WelcomeScreen({ onStart, onSkip }) {
    const [query, setQuery] = useState("");

    function handleSubmit(event) {
        event.preventDefault();
        const trimmed = query.trim();
        if (trimmed.length === 0) return;
        onStart(trimmed);
    }

    return (
        <section className="welcome">
            <div className="welcome-inner">
                {/*<span className="welcome-mark" aria-hidden="true">*/}
                {/*    <BookIcon />*/}
                {/*</span>*/}

                <h1 className="welcome-title">Codex</h1>

                <p className="welcome-subtitle">
                    Start with a question, a curious thought, or something you've always wanted to understand. Codex
                    will turn it into a lesson.
                </p>

                <form className="welcome-form" onSubmit={handleSubmit}>
                    <input
                        className="welcome-input"
                        type="text"
                        value={query}
                        onChange={(event) => setQuery(event.target.value)}
                        placeholder="What do you want to understand?"
                        autoFocus
                    />
                    <button className="welcome-submit" type="submit" aria-label="Start learning">
                        <ArrowIcon />
                    </button>
                </form>

                <div className="welcome-examples">
                    {PROMPT_EXAMPLES.map((example) => (
                        <button key={example} type="button" className="welcome-chip" onClick={() => onStart(example)}>
                            {example}
                        </button>
                    ))}
                </div>

                <button className="welcome-skip" type="button" onClick={onSkip}>
                    Skip to your lessons
                </button>
            </div>
        </section>
    );
}

export default WelcomeScreen;
