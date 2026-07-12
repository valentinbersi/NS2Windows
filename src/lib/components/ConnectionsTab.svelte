<script lang="ts">
    import {onDestroy, onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import type {UnlistenFn} from "@tauri-apps/api/event";
    import {listen} from "@tauri-apps/api/event";
    import type {Connection, LedPattern as LedPatternValue, NsInput} from "../types";
    import {CONTROLLER_KIND_LABELS, ControllerKind, LedPattern} from "../types";
    import {addConnection, connections, renameConnection} from "../stores";
    import {removeConnectionWithVirtualControllerCleanup} from "../connectionRemoval";

    import JoyConLeftIcon from "./icons/JoyConLeftIcon.svelte";
    import JoyConRightIcon from "./icons/JoyConRightIcon.svelte";
    import ProControllerIcon from "./icons/ProControllerIcon.svelte";
    import GcControllerIcon from "./icons/GcControllerIcon.svelte";

    import LeftJoyConInput from "./LeftJoyConInput.svelte";
    import RightJoyConInput from "./RightJoyConInput.svelte";
    import ProControllerInput from "./ProControllerInput.svelte";
    import GcControllerInput from "./GcControllerInput.svelte";

    // Connection Flow State
    let isConnecting = false;
    let connectingId: string | null = null;
    let detectedKind: ControllerKind | null = null;
    let detectedKinds: Record<string, ControllerKind> = {};

    let controllerInputs: Record<string, Partial<Record<NsInput, number>>> = {};
    let pendingLedUpdates: Record<string, boolean> = {};
    let nameDrafts: Record<string, string> = {};
    let nameErrors: Record<string, string> = {};

    const DEFAULT_LED_PATTERN = LedPattern.Led1;
    const LED_OPTIONS: {index: number; flag: LedPatternValue}[] = [
        {index: 1, flag: LedPattern.Led1},
        {index: 2, flag: LedPattern.Led2},
        {index: 3, flag: LedPattern.Led3},
        {index: 4, flag: LedPattern.Led4},
    ];

    let unlistenWaiting: UnlistenFn | null = null;
    let unlistenConfiguring: UnlistenFn | null = null;
    let unlistenInput: UnlistenFn | null = null;

    onMount(async () => {
        unlistenWaiting = await listen<string>("waiting-connection", event => {
            connectingId = event.payload;
        });

        unlistenConfiguring = await listen<[string, ControllerKind]>("configuring-connection", (event) => {
            const [id, kind] = event.payload;
            detectedKinds[id] = kind;
            detectedKinds = detectedKinds;

            if (id === connectingId) {
                detectedKind = kind;
            }
        });

        unlistenInput = await listen<[string, { inputs: Record<string, number> }]>("update-input", (event) => {
            const [id, inputData] = event.payload;
            controllerInputs = {
                ...controllerInputs,
                [id]: inputData.inputs as Partial<Record<NsInput, number>>,
            };
        });
    });

    onDestroy(() => {
        if (unlistenWaiting) unlistenWaiting();
        if (unlistenConfiguring) unlistenConfiguring();
        if (unlistenInput) unlistenInput();
    });

    function resetConnectionState() {
        isConnecting = false;
        connectingId = null;
        detectedKind = null;
    }

    async function removeConnection(id: string) {
        if (confirm("Are you sure you want to remove this connection?")) {
            try {
                await removeConnectionWithVirtualControllerCleanup(
                    id,
                    async () => {
                        await invoke("disconnect_controller", {id});
                    },
                );
                const {[id]: _removed, ...remainingInputs} = controllerInputs;
                controllerInputs = remainingInputs;
                const {[id]: _removedPendingLedUpdate, ...remainingPendingLedUpdates} = pendingLedUpdates;
                pendingLedUpdates = remainingPendingLedUpdates;
            } catch (e) {
                console.error("Failed to remove connection", e);
                alert("Failed to remove connection: " + e);
            }
        }
    }

    async function startConnectionFlow() {
        isConnecting = true;
        connectingId = null;
        detectedKind = null;
        try {
            const newId = await invoke<string>("connect_controller");
            const controllerKind = detectedKinds[newId];

            if (!controllerKind) {
                throw new Error("The connected controller type was not reported by the backend.");
            }

            addConnection({
                id: newId,
                controller_kind: controllerKind,
                led_pattern: DEFAULT_LED_PATTERN,
            });
            delete detectedKinds[newId];
            detectedKinds = detectedKinds;
            resetConnectionState();
        } catch (e) {
            console.error("Connection failed", e);
            // Just reset state if canceled or failed
            resetConnectionState();
        }
    }

    function updateConnectionLedPattern(id: string, ledPattern: LedPatternValue) {
        connections.update(conns => conns.map(conn => (
            conn.id === id ? {...conn, led_pattern: ledPattern} : conn
        )));
    }

    async function toggleConnectionLed(id: string, currentPattern: LedPatternValue, ledFlag: LedPatternValue) {
        const nextPattern = LedPattern.toggle(currentPattern, ledFlag);

        updateConnectionLedPattern(id, nextPattern);
        pendingLedUpdates = {...pendingLedUpdates, [id]: true};

        try {
            await invoke("set_controller_led", {id, ledPattern: LedPattern.bits(nextPattern)});
        } catch (e) {
            console.error("Failed to update controller LED pattern", e);
            updateConnectionLedPattern(id, currentPattern);
            alert("Failed to update controller LED pattern: " + e);
        } finally {
            const {[id]: _finished, ...remainingPendingLedUpdates} = pendingLedUpdates;
            pendingLedUpdates = remainingPendingLedUpdates;
        }
    }

    function clearNameDraft(id: string) {
        const {[id]: _draft, ...remainingDrafts} = nameDrafts;
        const {[id]: _error, ...remainingErrors} = nameErrors;
        nameDrafts = remainingDrafts;
        nameErrors = remainingErrors;
    }

    function updateNameDraft(id: string, event: Event) {
        nameDrafts = {
            ...nameDrafts,
            [id]: (event.currentTarget as HTMLInputElement).value,
        };

        if (nameErrors[id]) {
            const {[id]: _error, ...remainingErrors} = nameErrors;
            nameErrors = remainingErrors;
        }
    }

    function commitControllerName(connection: Connection) {
        const result = renameConnection(connection.id, nameDrafts[connection.id] ?? connection.name);

        if (result.ok) {
            clearNameDraft(connection.id);
            return;
        }

        nameErrors = {...nameErrors, [connection.id]: result.error};
    }

    function handleNameKeydown(connection: Connection, event: KeyboardEvent) {
        const input = event.currentTarget as HTMLInputElement;

        if (event.key === "Enter") {
            event.preventDefault();
            input.blur();
        } else if (event.key === "Escape") {
            event.preventDefault();
            clearNameDraft(connection.id);
            input.blur();
        }
    }

    function getIconForKind(kind: ControllerKind) {
        switch (kind) {
            case ControllerKind.LeftJoyCon:
                return JoyConLeftIcon;
            case ControllerKind.RightJoyCon:
                return JoyConRightIcon;
            case ControllerKind.ProController:
                return ProControllerIcon;
            case ControllerKind.NsoGcController:
                return GcControllerIcon;
            default:
                return ProControllerIcon;
        }
    }
</script>

<div class="tab-container">
    <div class="toolbar">
        <h2>Active Connections</h2>
        <button class="primary" on:click={startConnectionFlow}>
            + Add Connection
        </button>
    </div>

    {#if $connections.length === 0}
        <div class="empty-state">
            <p>No controllers currently connected.</p>
        </div>
    {:else}
        <div class="connections-list">
            {#each $connections as connection (connection.id)}
                <div class="connection-card">
                    <div class="connection-card-header">
                        <div class="icon-wrapper">
                            <svelte:component this={getIconForKind(connection.controller_kind)} width="32" height="32"/>
                        </div>
                        <div class="connection-info">
                            <input
                                class="controller-name-input"
                                class:invalid={Boolean(nameErrors[connection.id])}
                                value={nameDrafts[connection.id] ?? connection.name}
                                title="Edit controller name"
                                aria-label={`Name for ${connection.name}`}
                                aria-invalid={Boolean(nameErrors[connection.id])}
                                aria-describedby={nameErrors[connection.id] ? `controller-name-error-${connection.id}` : undefined}
                                on:input={(event) => updateNameDraft(connection.id, event)}
                                on:blur={() => commitControllerName(connection)}
                                on:keydown={(event) => handleNameKeydown(connection, event)}
                            />
                            {#if nameErrors[connection.id]}
                                <span
                                    id={`controller-name-error-${connection.id}`}
                                    class="controller-name-error"
                                    role="alert"
                                >{nameErrors[connection.id]}</span>
                            {/if}
                        </div>
                        <div class="connection-actions">
                            <button class="action-btn delete-btn" title="Remove Connection"
                                    on:click={() => removeConnection(connection.id)}>
                                🗑
                            </button>
                        </div>
                    </div>

                    <div class="led-control">
                        <span class="led-label">Player LEDs</span>
                        <div class="led-buttons" aria-label="Controller LED pattern">
                            {#each LED_OPTIONS as led}
                                <button
                                        class="led-button"
                                        class:active={LedPattern.contains(connection.led_pattern, led.flag)}
                                        disabled={pendingLedUpdates[connection.id]}
                                        title={`Toggle LED ${led.index}`}
                                        aria-label={`Toggle LED ${led.index}`}
                                        aria-pressed={LedPattern.contains(connection.led_pattern, led.flag)}
                                        on:click={() => toggleConnectionLed(connection.id, connection.led_pattern, led.flag)}
                                >
                                </button>
                            {/each}
                        </div>
                    </div>

                    <div class="input-display-container">
                        {#if connection.controller_kind === ControllerKind.LeftJoyCon}
                            <LeftJoyConInput inputs={controllerInputs[connection.id] || {}}/>
                        {:else if connection.controller_kind === ControllerKind.RightJoyCon}
                            <RightJoyConInput inputs={controllerInputs[connection.id] || {}}/>
                        {:else if connection.controller_kind === ControllerKind.ProController}
                            <ProControllerInput inputs={controllerInputs[connection.id] || {}}/>
                        {:else if connection.controller_kind === ControllerKind.NsoGcController}
                            <GcControllerInput inputs={controllerInputs[connection.id] || {}}/>
                        {/if}
                    </div>
                </div>
            {/each}
        </div>
    {/if}

    <!-- Connection Modal Overlay -->
    {#if isConnecting}
        <div class="modal-overlay">
            <div class="modal-content">
                <div class="waiting-state">
                    <div class="spinner"></div>
                    <h3>{detectedKind ? "Controller Detected" : "Waiting for Controller"}</h3>
                    {#if detectedKind}
                        <p class="waiting-prompt">
                            Configuring <strong>{CONTROLLER_KIND_LABELS[detectedKind]}</strong>.<br/>
                            Please Wait...
                        </p>
                        <div class="waiting-icon">
                            <svelte:component this={getIconForKind(detectedKind)} width="64" height="64"/>
                        </div>
                    {:else}
                        <p class="waiting-prompt">
                            Please hold the sync button on the controller you want to pair.
                        </p>
                    {/if}

                    <div class="modal-actions">
                        <button on:click={resetConnectionState}>Cancel</button>
                    </div>
                </div>
            </div>
        </div>
    {/if}
</div>

<style>
    .tab-container {
        display: flex;
        flex-direction: column;
        height: 100%;
        padding: 24px 32px;
        gap: 24px;
        overflow: hidden;
    }

    .toolbar {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .toolbar h2 {
        margin: 0;
        font-weight: 500;
    }

    .empty-state {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        color: var(--text-muted);
    }

    .connections-list {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
        gap: 16px;
        overflow-y: auto;
        padding-bottom: 24px;
    }

    .connection-card {
        display: flex;
        flex-direction: column;
        background: var(--bg-surface);
        border: 1px solid var(--border-color);
        border-radius: 8px;
        padding: 16px;
        gap: 16px;
        transition: all 0.2s ease;
    }

    .connection-card-header {
        display: flex;
        align-items: flex-start;
        gap: 16px;
    }

    .connection-card:hover {
        border-color: var(--text-muted);
    }

    .icon-wrapper {
        color: var(--text-muted);
        display: flex;
        align-items: center;
        justify-content: center;
        margin-top: 7px;
    }

    .connection-info {
        flex: 1;
        display: flex;
        flex-direction: column;
        gap: 4px;
    }

    .controller-name-input {
        width: 100%;
        min-width: 0;
        box-sizing: border-box;
        padding: 6px 8px;
        border: 1px solid transparent;
        border-radius: 6px;
        outline: none;
        background: transparent;
        font-size: 15px;
        font-weight: 500;
        color: var(--text-color);
    }

    .controller-name-input:hover {
        border-color: var(--border-color);
    }

    .controller-name-input:focus {
        border-color: var(--accent-color);
        background: var(--bg-color);
    }

    .controller-name-input.invalid {
        border-color: var(--danger-color, #ff4444);
    }

    .controller-name-error {
        color: var(--danger-color, #ff4444);
        font-size: 11px;
        line-height: 1.3;
    }

    /*.controller-id {*/
    /*    font-size: 12px;*/
    /*    color: var(--text-muted);*/
    /*}*/

    .input-display-container {
        display: flex;
        justify-content: center;
        align-items: center;
        padding: 16px;
        background: var(--bg-color);
        border-radius: 8px;
        min-width: 0;
        width: 100%;
    }

    .led-control {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 12px;
        padding: 10px 12px;
        background: var(--bg-color);
        border: 1px solid var(--border-color);
        border-radius: 8px;
    }

    .led-label {
        color: var(--text-muted);
        font-size: 13px;
        font-weight: 500;
    }

    .led-buttons {
        display: grid;
        grid-template-columns: repeat(4, 18px);
        gap: 7px;
    }

    .led-button {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 18px;
        height: 18px;
        padding: 0;
        border-radius: 3px;
        background: #050505;
        border: 1px solid #4a4a4a;
        box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.03);
    }

    .led-button:hover:not(:disabled):not(.active) {
        background: rgba(45, 185, 84, 0.28);
        border-color: rgba(105, 220, 124, 0.65);
        box-shadow: 0 0 8px rgba(45, 185, 84, 0.2);
    }

    .led-button.active {
        background: #32f36f;
        border-color: #89ff9f;
        box-shadow:
            0 0 10px rgba(50, 243, 111, 0.65),
            inset 0 0 3px rgba(255, 255, 255, 0.55);
    }

    .led-button.active:hover:not(:disabled) {
        background: #46ff80;
        border-color: #a5ffb4;
    }

    .led-button:disabled {
        cursor: wait;
        opacity: 0.75;
    }

    .connection-actions {
        display: flex;
    }

    .action-btn {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 32px;
        height: 32px;
        border-radius: 50%;
        background: transparent;
        border: none;
        color: var(--text-muted);
        font-size: 14px;
        transition: all 0.2s;
    }

    .action-btn:hover {
        color: var(--danger-color);
        background: rgba(255, 68, 68, 0.1);
    }

    /* Modal Styles */
    .modal-overlay {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background: rgba(0, 0, 0, 0.7);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
        backdrop-filter: blur(4px);
    }

    .modal-content {
        background: var(--bg-color);
        border: 1px solid var(--border-color);
        border-radius: 12px;
        padding: 32px;
        width: 100%;
        max-width: 500px;
        box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5);
        display: flex;
        flex-direction: column;
        gap: 20px;
    }

    .modal-content h3 {
        margin: 0;
        text-align: center;
        font-weight: 500;
    }

    .modal-actions {
        display: flex;
        justify-content: center;
        margin-top: 16px;
    }

    .waiting-state {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 16px;
        padding: 24px 0;
    }

    .waiting-prompt {
        text-align: center;
        color: var(--text-muted);
        line-height: 1.5;
    }

    .waiting-prompt strong {
        color: var(--text-color);
    }

    .waiting-icon {
        margin-top: 16px;
        animation: pulse 2s infinite ease-in-out;
    }

    .spinner {
        width: 40px;
        height: 40px;
        border: 3px solid var(--border-color);
        border-top-color: var(--accent-color);
        border-radius: 50%;
        animation: spin 1s linear infinite;
        margin-bottom: 8px;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }

    @keyframes pulse {
        0% {
            transform: scale(0.95);
            opacity: 0.5;
        }
        50% {
            transform: scale(1.05);
            opacity: 1;
        }
        100% {
            transform: scale(0.95);
            opacity: 0.5;
        }
    }
</style>
