<script lang="ts">
    import {type Input, NS_INPUT_LABELS, type NsInput} from "../types";
    import ExpressionInput from "./ExpressionInput.svelte";

    export let title: string = "Input Mapping";
    export let initialCondition: Input | null = null;
    export let onAccept: (cond: Input | null) => void;
    export let onCancel: () => void;

    let condition = initialCondition;
    let isValid = true;
    let expressionInputRef: ExpressionInput;

    const INPUT_GROUPS: { name: string, keys: NsInput[] }[] = [
        {
            name: "Face Buttons",
            keys: ["A", "B", "X", "Y"]
        },
        {
            name: "D-Pad",
            keys: ["Up", "Down", "Left", "Right"]
        },
        {
            name: "Menu Buttons",
            keys: ["Plus", "Minus", "Home", "Capture", "Chat"]
        },
        {
            name: "Shoulder & Triggers",
            keys: ["L", "R", "Zl", "Zr", "LTrigger", "RTrigger", "Tl", "Tr", "Sl", "Sr", "Gl", "Gr"]
        },
        {
            name: "Left Stick",
            keys: ["LeftXMinus", "LeftXPlus", "LeftYMinus", "LeftYPlus"]
        },
        {
            name: "Right Stick",
            keys: ["RightXMinus", "RightXPlus", "RightYMinus", "RightYPlus"]
        },
        {
            name: "Motion (Accel & Gyro)",
            keys: [
                "AccelUp", "AccelDown", "AccelLeft", "AccelRight", "AccelForward", "AccelBackward",
                "GyroPitchUp", "GyroPitchDown", "GyroRollLeft", "GyroRollRight", "GyroYawLeft", "GyroYawRight"
            ]
        }
    ];

    let expandedGroups: Record<string, boolean> = {};

    function toggleGroup(groupName: string) {
        expandedGroups[groupName] = !expandedGroups[groupName];
    }

    function insertToken(token: string) {
        if (expressionInputRef) {
            expressionInputRef.insertText(token);
        }
    }

    function handleAccept() {
        if (!isValid) return;
        onAccept(condition);
    }

    function handleClear() {
        if (expressionInputRef) {
            expressionInputRef.clear();
        }
    }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="modal-backdrop" on:click|self={onCancel}>
    <div class="modal-content">
        <div class="modal-header">
            <h2>{title}</h2>
            <button class="close-btn" on:click={onCancel}>✕</button>
        </div>

        <div class="modal-body">
            <div class="panels">
                <div class="left-panel">
                    <h3>Switch Inputs</h3>
                    <div class="input-groups-container">
                        {#each INPUT_GROUPS as group}
                            <div class="input-group">
                                <button class="group-header" on:click={() => toggleGroup(group.name)}>
                                    <span class="chevron" class:expanded={expandedGroups[group.name]}>▶</span>
                                    {group.name}
                                </button>
                                {#if expandedGroups[group.name]}
                                    <div class="group-items">
                                        {#each group.keys as key}
                                            <!-- @ts-ignore -->
                                            <button class="input-btn" on:click={() => insertToken(` ${key} `)}>
                                                <!-- @ts-ignore -->
                                                {NS_INPUT_LABELS[key]}
                                            </button>
                                        {/each}
                                    </div>
                                {/if}
                            </div>
                        {/each}
                    </div>
                </div>

                <div class="right-panel">
                    <h3>Logical Operators</h3>
                    <div class="operators-container">
                        <button class="operator-btn op-and" on:click={() => insertToken(' and ')}>AND</button>
                        <button class="operator-btn op-or" on:click={() => insertToken(' or ')}>OR</button>
                        <button class="operator-btn op-paren" on:click={() => insertToken(' ( ')}>(</button>
                        <button class="operator-btn op-paren" on:click={() => insertToken(' ) ')}>)</button>
                    </div>
                </div>
            </div>

            <div class="expression-section">
                <h3>Mapped Expression</h3>
                <ExpressionInput
                        bind:isValid
                        bind:this={expressionInputRef}
                        bind:value={condition}
                />
            </div>
        </div>

        <div class="modal-footer">
            <button class="secondary-btn" on:click={handleClear}>Clear</button>
            <div class="footer-right">
                <button on:click={onCancel}>Cancel</button>
                <button class="primary" disabled={!isValid} on:click={handleAccept}>Accept
                </button>
            </div>
        </div>
    </div>
</div>

<style>
    .modal-backdrop {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.6);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 1000;
        backdrop-filter: blur(2px);
    }

    .modal-content {
        background: var(--bg-surface, #1e1e1e);
        border: 1px solid var(--border-color, #333);
        border-radius: 8px;
        width: 90%;
        max-width: 800px;
        max-height: 90vh;
        display: flex;
        flex-direction: column;
        box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
    }

    .modal-header {
        padding: 16px 24px;
        border-bottom: 1px solid var(--border-color, #333);
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .modal-header h2 {
        margin: 0;
        font-size: 1.25rem;
        font-weight: 500;
    }

    .close-btn {
        background: transparent;
        border: none;
        color: var(--text-muted, #888);
        font-size: 1.2rem;
        cursor: pointer;
        padding: 4px;
    }

    .close-btn:hover {
        color: var(--text-color, #fff);
    }

    .modal-body {
        padding: 20px 24px;
        display: flex;
        flex-direction: column;
        gap: 20px;
        flex: 1;
        overflow: hidden;
    }

    .panels {
        display: flex;
        gap: 24px;
        flex: 1;
        min-height: 300px;
        overflow: hidden;
    }

    .left-panel, .right-panel {
        display: flex;
        flex-direction: column;
        gap: 12px;
    }

    .left-panel {
        flex: 2;
        overflow: hidden;
    }

    .right-panel {
        flex: 1;
    }

    h3 {
        margin: 0;
        font-size: 0.9rem;
        text-transform: uppercase;
        color: var(--text-muted, #888);
        letter-spacing: 0.5px;
    }

    .input-groups-container {
        flex: 1;
        overflow-y: auto;
        border: 1px solid var(--border-color, #333);
        border-radius: 6px;
        background: var(--bg-surface-hover, #232323);
    }

    .group-header {
        width: 100%;
        text-align: left;
        background: var(--bg-surface-hover, #2a2a2a);
        border: none;
        border-bottom: 1px solid var(--border-color, #333);
        padding: 10px 12px;
        font-weight: 500;
        color: var(--text-color, #eee);
        display: flex;
        align-items: center;
        gap: 8px;
        cursor: pointer;
    }

    .group-header:hover {
        background: #333;
    }

    .chevron {
        font-size: 0.7rem;
        transition: transform 0.2s;
    }

    .chevron.expanded {
        transform: rotate(90deg);
    }

    .group-items {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
        gap: 6px;
        padding: 10px;
        border-bottom: 1px solid var(--border-color, #333);
    }

    .input-btn {
        background: #2a2a2a;
        border: 1px solid #444;
        color: #98c379;
        padding: 6px 8px;
        border-radius: 4px;
        font-size: 0.85rem;
        cursor: pointer;
        transition: background 0.15s, border-color 0.15s;
    }

    .input-btn:hover {
        background: #3a3a3a;
        border-color: #98c379;
    }

    .operators-container {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 10px;
        background: var(--bg-surface-hover, #232323);
        padding: 12px;
        border: 1px solid var(--border-color, #333);
        border-radius: 6px;
    }

    .operator-btn {
        background: #2a2a2a;
        border: 1px solid #444;
        padding: 10px;
        border-radius: 4px;
        font-size: 0.9rem;
        font-weight: bold;
        cursor: pointer;
        transition: all 0.15s;
    }

    .operator-btn:hover {
        background: #3a3a3a;
    }

    .op-and {
        color: #c678dd;
    }

    .op-or {
        color: #c678dd;
    }

    .op-paren {
        color: #e5c07b;
    }

    .op-and:hover {
        border-color: #c678dd;
    }

    .op-or:hover {
        border-color: #c678dd;
    }

    .op-paren:hover {
        border-color: #e5c07b;
    }

    .expression-section {
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .modal-footer {
        padding: 16px 24px;
        border-top: 1px solid var(--border-color, #333);
        display: flex;
        justify-content: space-between;
        align-items: center;
        background: var(--bg-surface-hover, #232323);
        border-radius: 0 0 8px 8px;
    }

    .footer-right {
        display: flex;
        gap: 12px;
    }

    .secondary-btn {
        background: #2a2a2a;
        color: #e06c75;
        border: 1px solid #444;
    }

    .secondary-btn:hover {
        background: #3a3a3a;
        border-color: #e06c75;
    }
</style>
