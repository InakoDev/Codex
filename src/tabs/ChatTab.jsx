import { useEffect, useRef, useState } from "react";
import { ArrowIcon } from "../components/Icons.jsx";

import "../styles/components/chat.css";

function createId() {
    return Math.random().toString(36).slice(2, 9);
}

function buildOpeningExchange(query) {
    return [
        { id: createId(), role: "user", text: query },
        {
            id: createId(),
            role: "assistant",
            text: "Before we get started, here's a high level roadmap before we go deeper:",
        },
        {
            id: createId(),
            role: "diagram",
            label: "Roadmap",
            caption:
                "A rough map of how the pieces connect. Real diagrams will render here once everything is wired up.",
        },
        {
            id: createId(),
            role: "assistant",
            text: "Would you like to proceed with this?",
        },
    ];
}

function ChatTab({ initialQuery }) {
    const [messages, setMessages] = useState(() => (initialQuery ? buildOpeningExchange(initialQuery) : []));
    const [draft, setDraft] = useState("");
    const logRef = useRef(null);

    useEffect(() => {
        if (logRef.current) {
            logRef.current.scrollTop = logRef.current.scrollHeight;
        }
    }, [messages]);

    function handleSend(event) {
        event.preventDefault();
        const trimmed = draft.trim();
        if (trimmed.length === 0) return;

        setMessages((prev) => [
            ...prev,
            { id: createId(), role: "user", text: trimmed },
            {
                id: createId(),
                role: "assistant",
                text: "This reply is a placeholder until Codex is wired up to a model.",
            },
        ]);

        setDraft("");
    }

    if (messages.length === 0) {
        return (
            <section className="chat-tab chat-empty">
                <p>Ask a question from the welcome screen or a lesson to start a conversation.</p>
            </section>
        );
    }

    return (
        <section className="chat-tab">
            <div className="chat-log" ref={logRef}>
                {messages.map((message) => {
                    if (message.role === "diagram") {
                        return (
                            <div key={message.id} className="chat-diagram">
                                <span className="chat-diagram-label">{message.label}</span>
                                <p>{message.caption}</p>
                            </div>
                        );
                    }
                    return (
                        <div key={message.id} className={`chat-message chat-message-${message.role}`}>
                            <p>{message.text}</p>
                        </div>
                    );
                })}
            </div>

            <form className="chat-composer" onSubmit={handleSend}>
                <input
                    type="text"
                    value={draft}
                    onChange={(event) => setDraft(event.target.value)}
                    placeholder="Ask a follow up question..."
                />
                <button type="submit" aria-label="Send message">
                    <ArrowIcon />
                </button>
            </form>
        </section>
    );
}

export default ChatTab;
