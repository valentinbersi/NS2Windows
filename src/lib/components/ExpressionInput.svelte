<script lang="ts">
    import type { Input } from '../types';
    import { NS_INPUT_LABELS } from '../types';
    import { parseExpression, stringifyCondition } from '../utils/expressionParser';
    import { onMount, tick } from 'svelte';

    export let value: Input | null = null;
    export let isValid: boolean = true;
    export let id: string = "";
    export let readonly: boolean = false;

    let text = !value ? "" : stringifyCondition(value);
    let prevValue = value;
    let scrollLeft = 0;
    let inputEl: HTMLInputElement;
    let lastCursorPos: number = 0;

    // Track cursor position on every interaction so we have it even after focus loss
    function saveCursorPos() {
        if (inputEl && document.activeElement === inputEl) {
            lastCursorPos = inputEl.selectionEnd ?? text.length;
        }
    }

    export async function insertText(insertStr: string) {
        if (readonly) return;
        const pos = lastCursorPos;
        text = text.substring(0, pos) + insertStr + text.substring(pos);
        const newPos = pos + insertStr.length;
        lastCursorPos = newPos;

        // Trigger validation
        updateFromText(text);

        // Wait for Svelte to flush the DOM, then restore cursor & scroll
        await tick();
        if (inputEl) {
            inputEl.focus();
            inputEl.selectionStart = inputEl.selectionEnd = newPos;
            // Force the browser to scroll the input so the cursor is visible
            // by briefly making text color visible (no-op trick) then reading scrollLeft
            requestAnimationFrame(() => {
                if (inputEl) {
                    scrollLeft = inputEl.scrollLeft;
                }
            });
        }
    }

    export function clear() {
        if (readonly) return;
        text = "";
        updateFromText(text);
    }

    const VALID_INPUTS = new Set(Object.keys(NS_INPUT_LABELS));

    // When value changes from the outside
    $: if (value !== prevValue) {
        if (!value) {
            if (isValid) text = "";
        } else {
            text = stringifyCondition(value);
        }
        prevValue = value;
    }

    function handleInput(e: Event) {
        handleScroll(e);
        saveCursorPos();
        updateFromText(text);
    }

    function updateFromText(newText: string) {
        if (newText.trim() === "") {
            isValid = true;
            if (value !== null) {
                value = null;
                prevValue = value;
            }
        } else {
            const parsed = parseExpression(newText);
            if (parsed === null) {
                isValid = false;
                if (value !== null) {
                    value = null;
                    prevValue = null;
                }
            } else {
                isValid = true;
                value = parsed;
                prevValue = value;
            }
        }
    }

    // Highlighter
    $: highlightedHtml = highlightText(text);

    function highlightText(t: string): string {
        const tokens = t.split(/(\(|\)|\band\b|\bor\b|\s+)/gi);
        
        return tokens.map(token => {
            if (!token) return '';
            
            const lower = token.toLowerCase();
            if (lower === 'and' || lower === 'or') {
                return `<span style="color: #c678dd;">${escapeHtml(token)}</span>`;
            } else if (token === '(' || token === ')') {
                return `<span style="color: #e5c07b;">${escapeHtml(token)}</span>`;
            } else if (token.trim() === '') {
                return token; // Spaces
            } else {
                const isValidInput = Array.from(VALID_INPUTS).some(v => v.toLowerCase() === lower);
                if (isValidInput) {
                    return `<span style="color: #98c379;">${escapeHtml(token)}</span>`;
                } else {
                    return `<span style="color: #e06c75; text-decoration: underline;">${escapeHtml(token)}</span>`;
                }
            }
        }).join('');
    }

    function escapeHtml(unsafe: string) {
        return unsafe
             .replace(/&/g, "&amp;")
             .replace(/</g, "&lt;")
             .replace(/>/g, "&gt;")
             .replace(/"/g, "&quot;")
             .replace(/'/g, "&#039;");
    }

    function handleScroll(e: Event) {
        scrollLeft = (e.target as HTMLInputElement).scrollLeft;
    }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="expression-container" class:invalid={!isValid} class:is-readonly={readonly} on:click>
    <div class="highlighter" aria-hidden="true" style="transform: translateX(-{scrollLeft}px)">
        {@html highlightedHtml}
    </div>
    <input
        bind:this={inputEl}
        {id}
        type="text"
        bind:value={text}
        on:scroll={handleScroll}
        on:input={handleInput}
        on:keyup={saveCursorPos}
        on:mouseup={saveCursorPos}
        class="real-input"
        spellcheck="false"
        autocomplete="off"
        placeholder={readonly ? "Unmapped" : "e.g. (A or B) and Y"}
        {readonly}
    />
</div>

<style>
    .expression-container {
        position: relative;
        width: 100%;
        height: 38px;
        overflow: hidden;
        background: var(--bg-surface, #232323);
        border: 1px solid var(--border-color, #444);
        border-radius: 6px;
        transition: border-color 0.2s;
    }

    .expression-container:focus-within {
        border-color: #61afef;
        box-shadow: 0 0 0 2px rgba(97, 175, 239, 0.2);
    }

    .expression-container.invalid {
        border-color: #e06c75;
    }

    .expression-container.invalid:focus-within {
        box-shadow: 0 0 0 2px rgba(224, 108, 117, 0.2);
    }

    .expression-container.is-readonly {
        cursor: pointer;
        background: var(--bg-surface-hover, #2a2a2a);
    }

    .expression-container.is-readonly:hover {
        border-color: #61afef;
    }

    .expression-container.is-readonly .real-input {
        cursor: pointer;
    }

    .highlighter {
        position: absolute;
        top: 0;
        left: 0;
        width: max-content;
        min-width: 100%;
        height: 100%;
        padding: 8px 12px;
        font-family: inherit;
        font-size: 14px;
        line-height: 20px;
        white-space: pre;
        color: transparent;
        pointer-events: none;
        z-index: 1;
    }

    .real-input {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        padding: 8px 12px;
        font-family: inherit;
        font-size: 14px;
        line-height: 20px;
        background: transparent;
        color: transparent; /* Makes the original text invisible */
        caret-color: #fff; /* But cursor remains visible */
        border: none;
        outline: none;
        z-index: 2;
    }

    /* Override the placeholder color so it's visible */
    .real-input::placeholder {
        color: #777;
    }
</style>
