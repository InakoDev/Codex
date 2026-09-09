// Thought this would be better than making an icon font, might not be that scalable, but I'm unsure if I'd need a lot anyway.
// It'll do for now.

export function BookIcon(props) {
    return (
        <svg viewBox="0 0 18 18" fill="none" xmlns="http://www.w3.org/2000/svg" {...props}>
            <path
                d="M9 4.4c-1.1-.9-2.7-1.4-4.4-1.4-.6 0-1.1.44-1.1 1v8.6c0 .56.5 1 1.1 1 1.7 0 3.3.5 4.4 1.4m0-10.6c1.1-.9 2.7-1.4 4.4-1.4.6 0 1.1.44 1.1 1v8.6c0 .56-.5 1-1.1 1-1.7 0-3.3.5-4.4 1.4m0-10.6V15"
                stroke="currentColor"
                strokeWidth="1.3"
                strokeLinecap="round"
                strokeLinejoin="round"
            />
        </svg>
    );
}

export function ChatIcon(props) {
    return (
        <svg viewBox="0 0 18 18" fill="none" xmlns="http://www.w3.org/2000/svg" {...props}>
            <path
                d="M3 4.6h12a1 1 0 0 1 1 1v6.2a1 1 0 0 1-1 1H8.3l-3.1 2.3a.4.4 0 0 1-.64-.32v-1.98H3a1 1 0 0 1-1-1V5.6a1 1 0 0 1 1-1Z"
                stroke="currentColor"
                strokeWidth="1.3"
                strokeLinejoin="round"
            />
        </svg>
    );
}

export function LessonsIcon(props) {
    return (
        <svg viewBox="0 0 18 18" fill="none" xmlns="http://www.w3.org/2000/svg" {...props}>
            <rect x="3" y="2.8" width="12" height="12.4" rx="1.4" stroke="currentColor" strokeWidth="1.3" />
            <path d="M6 6.4h6M6 9h6M6 11.6h3.5" stroke="currentColor" strokeWidth="1.3" strokeLinecap="round" />
        </svg>
    );
}

export function ProgressIcon(props) {
    return (
        <svg viewBox="0 0 18 18" fill="none" xmlns="http://www.w3.org/2000/svg" {...props}>
            <path
                d="M3.5 14.5v-4M8 14.5V5.5M12.5 14.5V8.5"
                stroke="currentColor"
                strokeWidth="1.5"
                strokeLinecap="round"
            />
        </svg>
    );
}

export function ArrowIcon(props) {
    return (
        <svg viewBox="0 0 18 18" fill="none" xmlns="http://www.w3.org/2000/svg" {...props}>
            <path
                d="M4 9h9.5M9.5 4.5 14 9l-4.5 4.5"
                stroke="currentColor"
                strokeWidth="1.5"
                strokeLinecap="round"
                strokeLinejoin="round"
            />
        </svg>
    );
}
