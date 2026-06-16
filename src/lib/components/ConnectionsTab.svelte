<script lang="ts">
    import {onDestroy, onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import type {UnlistenFn} from "@tauri-apps/api/event";
    import {listen} from "@tauri-apps/api/event";
    import type {NsInput} from "../types";
    import {CONTROLLER_KIND_LABELS, ControllerKind} from "../types";
    import {connections} from "../stores";

    import JoyConLeftIcon from "./icons/JoyConLeftIcon.svelte";
    import JoyConRightIcon from "./icons/JoyConRightIcon.svelte";
    import ProControllerIcon from "./icons/ProControllerIcon.svelte";

    import LeftJoyConInput from "./LeftJoyConInput.svelte";
    import RightJoyConInput from "./RightJoyConInput.svelte";
    import ProControllerInput from "./ProControllerInput.svelte";
    import GcControllerInput from "./GcControllerInput.svelte";

    // Connection Flow State
    let isConnecting = false;
    let selectedKind: ControllerKind | null = null;
    let waitingFor: ControllerKind | null = null;
    let configuringFor: ControllerKind | null = null;

    let controllerInputs: Record<string, Partial<Record<NsInput, number>>> = {};

    let unlistenWaiting: UnlistenFn | null = null;
    let unlistenConfiguring: UnlistenFn | null = null;
    let unlistenInput: UnlistenFn | null = null;

    onMount(async () => {
        unlistenWaiting = await listen<[string, ControllerKind]>("waiting-connection", (event) => {
            waitingFor = event.payload[1];
        });

        unlistenConfiguring = await listen<[string, ControllerKind]>("configuring-connection", (event) => {
            configuringFor = event.payload[1];
        });

        unlistenInput = await listen<[string, { inputs: Record<string, number> }]>("update-input", (event) => {
            const [id, inputData] = event.payload;
            controllerInputs[id] = inputData.inputs as Partial<Record<NsInput, number>>;
        });
    });

    onDestroy(() => {
        if (unlistenWaiting) unlistenWaiting();
        if (unlistenConfiguring) unlistenConfiguring();
        if (unlistenInput) unlistenInput();
    });

    function resetConnectionState() {
        isConnecting = false;
        selectedKind = null;
        waitingFor = null;
        configuringFor = null;
    }

    async function removeConnection(id: string) {
        if (confirm("Are you sure you want to remove this connection?")) {
            try {
                await invoke("disconnect_controller", {id});
                connections.update(conns => conns.filter(c => c.id !== id));
                delete controllerInputs[id];
                controllerInputs = controllerInputs; // trigger reactivity
            } catch (e) {
                console.error("Failed to remove connection", e);
            }
        }
    }

    function startConnectionFlow() {
        isConnecting = true;
        selectedKind = null;
        waitingFor = null;
        configuringFor = null;
    }

    async function selectKindAndConnect(kind: ControllerKind) {
        selectedKind = kind;
        try {
            const new_id = await invoke<string>("connect_controller", {kind});
            connections.update(conns => [...conns, {id: new_id, controller_kind: kind}]);
            resetConnectionState();
        } catch (e) {
            console.error("Connection failed", e);
            // Just reset state if canceled or failed
            resetConnectionState();
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
                return ProControllerIcon;
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
                            <span class="controller-name">{CONTROLLER_KIND_LABELS[connection.controller_kind]}</span>
                            <span class="controller-id">ID: {connection.id.split('-')[0]}...</span>
                        </div>
                        <div class="connection-actions">
                            <button class="action-btn delete-btn" title="Remove Connection"
                                    on:click={() => removeConnection(connection.id)}>
                                🗑
                            </button>
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
                {#if selectedKind === null}
                    <h3>Select Controller to Connect</h3>
                    <p class="subtitle">Choose the type of device you want to pair.</p>

                    <div class="device-options">
                        {#each Object.values(ControllerKind) as kind}
                            <button class="device-btn" on:click={() => selectKindAndConnect(kind)}>
                                <svelte:component this={getIconForKind(kind)} width="48" height="48"/>
                                <span>{CONTROLLER_KIND_LABELS[kind]}</span>
                            </button>
                        {/each}
                    </div>

                    <div class="modal-actions">
                        <button on:click={resetConnectionState}>Cancel</button>
                    </div>
                {:else}
                    <!-- Waiting/Configuring State -->
                    <div class="waiting-state">
                        <div class="spinner"></div>
                        <h3>Pairing Device...</h3>
                        {#if configuringFor}
                            <p class="waiting-prompt">
                                Configuring <strong>{CONTROLLER_KIND_LABELS[configuringFor]}</strong>.<br/>
                                Please wait...
                            </p>
                            <div class="waiting-icon">
                                <svelte:component this={getIconForKind(configuringFor)} width="64" height="64"
                                                  fill="var(--accent-color)"/>
                            </div>
                        {:else if waitingFor}
                            <p class="waiting-prompt">
                                Waiting for <strong>{CONTROLLER_KIND_LABELS[waitingFor]}</strong>.<br/>
                                Please press any button on the device to pair it.
                            </p>
                            <div class="waiting-icon">
                                <svelte:component this={getIconForKind(waitingFor)} width="64" height="64"
                                                  fill="var(--accent-color)"/>
                            </div>
                        {:else}
                            <p class="waiting-prompt">Initializing connection...</p>
                        {/if}

                        <div class="modal-actions">
                            <button on:click={resetConnectionState}>Cancel</button>
                        </div>
                    </div>
                {/if}
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
        align-items: center;
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
    }

    .connection-info {
        flex: 1;
        display: flex;
        flex-direction: column;
        gap: 4px;
    }

    .controller-name {
        font-size: 15px;
        font-weight: 500;
        color: var(--text-color);
    }

    .controller-id {
        font-size: 12px;
        color: var(--text-muted);
    }

    .input-display-container {
        display: flex;
        justify-content: center;
        align-items: center;
        padding: 16px;
        background: var(--bg-color);
        border-radius: 8px;
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

    .subtitle {
        text-align: center;
        color: var(--text-muted);
        margin: 0 0 12px 0;
        font-size: 14px;
    }

    .device-options {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 16px;
    }

    .device-btn {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 12px;
        padding: 24px 16px;
        background: var(--bg-surface);
        border: 1px solid var(--border-color);
        border-radius: 8px;
        transition: all 0.2s;
        color: var(--text-muted);
    }

    .device-btn:hover {
        border-color: var(--accent-color);
        color: var(--text-color);
        background: var(--bg-surface-hover);
        transform: translateY(-2px);
    }

    .device-btn span {
        font-weight: 500;
        font-size: 14px;
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
