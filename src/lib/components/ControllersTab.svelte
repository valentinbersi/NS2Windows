<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { ControllerKind, ProfileKind, CONTROLLER_KIND_LABELS } from "../types";
    import type { Connection, VirtualControllerState, EmulatedController } from "../types";
    import { connections, virtualControllers } from "../stores";
    
    import JoyConLeftIcon from "./icons/JoyConLeftIcon.svelte";
    import JoyConRightIcon from "./icons/JoyConRightIcon.svelte";
    import ProControllerIcon from "./icons/ProControllerIcon.svelte";
    import GcControllerIcon from "./icons/GcControllerIcon.svelte";

    let profiles: string[] = [];
    let loading = true;

    // Selection state
    let selectingForVcId: string | null = null;

    async function loadData() {
        try {
            profiles = await invoke<string[]>("profile_names");
        } catch (e) {
            console.error("Failed to load profiles", e);
        } finally {
            loading = false;
        }
    }

    onMount(() => {
        loadData();
    });

    $: availableConnections = $connections.filter(
        c => !$virtualControllers.some(vc => vc.bound_controllers.some(bc => bc.id === c.id))
    );

    function addVirtualController() {
        const newVc: VirtualControllerState = {
            id: crypto.randomUUID(),
            profile_name: null,
            bound_controllers: [],
            is_running: false,
            emulated_controller_id: null,
            motion_source: "Right"
        };
        $virtualControllers = [...$virtualControllers, newVc];
    }

    function removeVirtualController(id: string) {
        $virtualControllers = $virtualControllers.filter(vc => vc.id !== id);
        if (selectingForVcId === id) {
            selectingForVcId = null;
        }
    }

    function openSelectionModal(vcId: string) {
        selectingForVcId = vcId;
    }

    function closeSelectionModal() {
        selectingForVcId = null;
    }

    function selectControllerForVc(conn: Connection) {
        if (!selectingForVcId) return;

        $virtualControllers = $virtualControllers.map(vc => {
            if (vc.id === selectingForVcId) {
                // Cannot bind if running
                if (vc.is_running) return vc;

                // Check if we can bind
                if (vc.bound_controllers.length >= 2) return vc; // full
                if (vc.bound_controllers.length === 1) {
                    const existing = vc.bound_controllers[0];
                    const isDualCapable = (existing.controller_kind === ControllerKind.LeftJoyCon && conn.controller_kind === ControllerKind.RightJoyCon) ||
                                          (existing.controller_kind === ControllerKind.RightJoyCon && conn.controller_kind === ControllerKind.LeftJoyCon);
                    if (!isDualCapable) {
                        alert("You can only combine a Left Joy-Con with a Right Joy-Con.");
                        return vc;
                    }
                }
                return { ...vc, bound_controllers: [...vc.bound_controllers, conn] };
            }
            return vc;
        });

        closeSelectionModal();
    }

    function unbindController(vcId: string, connId: string) {
        $virtualControllers = $virtualControllers.map(vc => {
            if (vc.id === vcId) {
                return { ...vc, bound_controllers: vc.bound_controllers.filter(c => c.id !== connId) };
            }
            return vc;
        });
    }

    function setProfile(vcId: string, profileName: string) {
        $virtualControllers = $virtualControllers.map(vc => 
            vc.id === vcId ? { ...vc, profile_name: profileName } : vc
        );
    }

    function setMotionSource(vcId: string, source: "Left" | "Right") {
        $virtualControllers = $virtualControllers.map(vc => 
            vc.id === vcId ? { ...vc, motion_source: source } : vc
        );
    }

    async function toggleEmulation(vc: VirtualControllerState) {
        if (vc.is_running) {
            // Stop
            try {
                await invoke("stop_controller", { emulatedControllerId: vc.emulated_controller_id });
                $virtualControllers = $virtualControllers.map(v => 
                    v.id === vc.id ? { ...v, is_running: false, emulated_controller_id: null } : v
                );
            } catch (e) {
                console.error(e);
                alert("Failed to stop emulation: " + e);
            }
        } else {
            // Start
            if (!vc.profile_name || vc.bound_controllers.length === 0) return;
            
            let connected_controller: any;
            if (vc.bound_controllers.length === 1) {
                connected_controller = { SingleController: { id: vc.bound_controllers[0].id } };
            } else if (vc.bound_controllers.length === 2) {
                const left = vc.bound_controllers.find(c => c.controller_kind === ControllerKind.LeftJoyCon);
                const right = vc.bound_controllers.find(c => c.controller_kind === ControllerKind.RightJoyCon);
                if (!left || !right) return;
                connected_controller = { 
                    DualJoyCon: { 
                        left_id: left.id, 
                        right_id: right.id, 
                        motion_source: vc.motion_source 
                    } 
                };
            }

            const payload: EmulatedController = {
                profile_name: vc.profile_name,
                connected_controller
            };

            try {
                const uuid = await invoke<string>("start_controller", { controller: payload });
                $virtualControllers = $virtualControllers.map(v => 
                    v.id === vc.id ? { ...v, is_running: true, emulated_controller_id: uuid } : v
                );
            } catch (e) {
                console.error(e);
                alert("Failed to start emulation: " + e);
            }
        }
    }

    function getControllerIcon(kind: ControllerKind) {
        switch (kind) {
            case ControllerKind.LeftJoyCon: return JoyConLeftIcon;
            case ControllerKind.RightJoyCon: return JoyConRightIcon;
            case ControllerKind.ProController: return ProControllerIcon;
            case ControllerKind.NsoGcController: return GcControllerIcon;
            default: return ProControllerIcon;
        }
    }
</script>

<div class="layout">
    <div class="available-panel">
        <h3>Connected Devices</h3>
        <p class="subtitle">Overview of available controllers</p>
        <div class="connections-list">
            {#if loading}
                <div class="empty-msg">Loading...</div>
            {:else if availableConnections.length === 0}
                <div class="empty-msg">No controllers available. Please connect one first.</div>
            {:else}
                {#each availableConnections as conn (conn.id)}
                    <div class="connection-item">
                        <svelte:component this={getControllerIcon(conn.controller_kind)} width="24" height="24" />
                        <span>{CONTROLLER_KIND_LABELS[conn.controller_kind]}</span>
                    </div>
                {/each}
            {/if}
        </div>
    </div>

    <div class="main-panel">
        <div class="toolbar">
            <h2>Virtual Controllers</h2>
            <button class="add-btn" on:click={addVirtualController}>+ Add Virtual Controller</button>
        </div>

        <div class="vc-list">
            {#if $virtualControllers.length === 0}
                <div class="empty-msg">No virtual controllers yet. Add one to start.</div>
            {:else}
                {#each $virtualControllers as vc (vc.id)}
                    <div class="vc-card" class:running={vc.is_running}>
                        <div class="vc-header">
                            <select 
                                value={vc.profile_name || ""} 
                                disabled={vc.is_running}
                                on:change={(e) => setProfile(vc.id, e.currentTarget.value)}
                            >
                                <option value="" disabled>Select a Profile</option>
                                {#each profiles as profileName}
                                    <option value={profileName}>{profileName}</option>
                                {/each}
                            </select>
                            <button 
                                class="delete-btn" 
                                disabled={vc.is_running} 
                                on:click={() => removeVirtualController(vc.id)}
                                title="Delete Virtual Controller"
                            >✕</button>
                        </div>

                        <div class="binding-zones" class:disabled={vc.is_running}>
                            {#each vc.bound_controllers as bound (bound.id)}
                                <div class="bound-controller">
                                    <svelte:component this={getControllerIcon(bound.controller_kind)} width="48" height="48" />
                                    <span class="label">{CONTROLLER_KIND_LABELS[bound.controller_kind]}</span>
                                    {#if !vc.is_running}
                                        <button class="unbind-btn" on:click={() => unbindController(vc.id, bound.id)}>✕</button>
                                    {/if}
                                </div>
                            {/each}

                            {#if vc.bound_controllers.length === 0}
                                <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
                                <div class="drop-placeholder" on:click={() => !vc.is_running && openSelectionModal(vc.id)}>
                                    <div class="dashed-box">Click to select a controller</div>
                                </div>
                            {:else if vc.bound_controllers.length === 1 && (vc.bound_controllers[0].controller_kind === ControllerKind.LeftJoyCon || vc.bound_controllers[0].controller_kind === ControllerKind.RightJoyCon)}
                                <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
                                <div class="drop-placeholder" on:click={() => !vc.is_running && openSelectionModal(vc.id)}>
                                    <div class="dashed-box">Click to select opposite Joy-Con</div>
                                </div>
                            {/if}
                        </div>

                        {#if vc.bound_controllers.length === 2}
                            <div class="motion-toggle">
                                <span>Motion Source:</span>
                                <div class="toggle-group">
                                    <button 
                                        class:active={vc.motion_source === "Left"} 
                                        disabled={vc.is_running} 
                                        on:click={() => setMotionSource(vc.id, "Left")}
                                    >Left</button>
                                    <button 
                                        class:active={vc.motion_source === "Right"} 
                                        disabled={vc.is_running} 
                                        on:click={() => setMotionSource(vc.id, "Right")}
                                    >Right</button>
                                </div>
                            </div>
                        {/if}

                        <div class="vc-footer">
                            <button 
                                class="start-stop-btn" 
                                class:stop={vc.is_running}
                                disabled={!vc.profile_name || vc.bound_controllers.length === 0}
                                on:click={() => toggleEmulation(vc)}
                            >
                                {#if vc.is_running}
                                    <span class="stop-icon">■</span> Stop Emulation
                                {:else}
                                    <span class="start-icon">▶</span> Start Emulation
                                {/if}
                            </button>
                        </div>
                    </div>
                {/each}
            {/if}
        </div>
    </div>
</div>

{#if selectingForVcId !== null}
    <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
    <div class="modal-backdrop" on:click={closeSelectionModal}>
        <div class="modal-content" on:click|stopPropagation>
            <h3>Select a Controller</h3>
            <p>Choose a device to bind to this Virtual Controller.</p>
            
            <div class="modal-grid">
                {#if availableConnections.length === 0}
                    <div class="empty-msg">No controllers available to bind. Please connect one.</div>
                {:else}
                    {#each availableConnections as conn (conn.id)}
                        <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
                        <div class="modal-card" on:click={() => selectControllerForVc(conn)}>
                            <svelte:component this={getControllerIcon(conn.controller_kind)} width="48" height="48" />
                            <span>{CONTROLLER_KIND_LABELS[conn.controller_kind]}</span>
                        </div>
                    {/each}
                {/if}
            </div>

            <div class="modal-actions">
                <button class="cancel-btn" on:click={closeSelectionModal}>Cancel</button>
            </div>
        </div>
    </div>
{/if}

<style>
    .layout {
        display: flex;
        height: 100%;
        gap: 24px;
        padding: 24px 32px;
        box-sizing: border-box;
        overflow: hidden;
    }

    .available-panel {
        width: 280px;
        background: var(--bg-surface);
        border: 1px solid var(--border-color);
        border-radius: 12px;
        padding: 20px;
        display: flex;
        flex-direction: column;
        flex-shrink: 0;
    }

    .available-panel h3 {
        margin: 0;
        font-size: 16px;
        font-weight: 500;
        color: var(--text-color);
    }

    .available-panel .subtitle {
        margin: 4px 0 16px 0;
        font-size: 12px;
        color: var(--text-muted);
    }

    .connections-list {
        display: flex;
        flex-direction: column;
        gap: 12px;
        overflow-y: auto;
        flex: 1;
        padding-right: 4px;
    }

    .connection-item {
        display: flex;
        align-items: center;
        gap: 12px;
        padding: 12px 16px;
        background: var(--bg-color);
        border: 1px solid var(--border-color);
        border-radius: 8px;
    }

    .connection-item span {
        font-size: 14px;
        font-weight: 500;
        color: var(--text-color);
    }

    .main-panel {
        flex: 1;
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .toolbar {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 20px;
        flex-shrink: 0;
    }

    .toolbar h2 {
        margin: 0;
        font-weight: 500;
        color: var(--text-color);
    }

    .add-btn {
        background: var(--accent-color);
        color: #000000;
        border: none;
        padding: 10px 16px;
        border-radius: 8px;
        font-weight: 500;
        cursor: pointer;
        transition: opacity 0.2s;
    }

    .add-btn:hover {
        opacity: 0.9;
    }

    .vc-list {
        display: flex;
        flex-direction: column;
        gap: 20px;
        overflow-y: auto;
        flex: 1;
        padding-right: 8px;
    }

    .vc-card {
        background: var(--bg-surface);
        border: 1px solid var(--border-color);
        border-radius: 12px;
        padding: 20px;
        display: flex;
        flex-direction: column;
        gap: 16px;
        transition: border-color 0.3s;
    }

    .vc-card.running {
        border-color: var(--success-color, #4caf50);
        box-shadow: 0 0 10px rgba(76, 175, 80, 0.1);
    }

    .vc-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 16px;
    }

    .vc-header select {
        flex: 1;
        padding: 10px;
        padding-right: 32px;
        border-radius: 8px;
        border: 1px solid var(--border-color);
        background: var(--bg-surface, #ffffff);
        color: var(--text-color, #000000);
        font-size: 14px;
        outline: none;
    }

    .vc-header select option {
        color: #000000;
        background: #ffffff;
    }

    .vc-header select:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .delete-btn {
        background: transparent;
        border: none;
        color: var(--text-muted);
        font-size: 18px;
        cursor: pointer;
        width: 32px;
        height: 32px;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 50%;
        transition: all 0.2s;
    }

    .delete-btn:hover:not(:disabled) {
        color: var(--danger-color, #ff4444);
        background: rgba(255, 68, 68, 0.1);
    }

    .delete-btn:disabled {
        opacity: 0.3;
        cursor: not-allowed;
    }

    .binding-zones {
        display: flex;
        gap: 16px;
        min-height: 120px;
    }

    .binding-zones.disabled {
        opacity: 0.8;
        pointer-events: none;
    }

    .bound-controller {
        flex: 1;
        background: var(--bg-color);
        border: 1px solid var(--border-color);
        border-radius: 8px;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 12px;
        position: relative;
        padding: 16px;
    }

    .bound-controller .label {
        font-size: 13px;
        font-weight: 500;
        color: var(--text-color);
    }

    .unbind-btn {
        position: absolute;
        top: 8px;
        right: 8px;
        background: rgba(0, 0, 0, 0.5);
        color: white;
        border: none;
        border-radius: 50%;
        width: 24px;
        height: 24px;
        font-size: 12px;
        display: flex;
        align-items: center;
        justify-content: center;
        cursor: pointer;
        transition: background 0.2s;
    }

    .unbind-btn:hover {
        background: var(--danger-color, #ff4444);
    }

    .drop-placeholder {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        cursor: pointer;
    }

    .dashed-box {
        width: 100%;
        height: 100%;
        border: 2px dashed var(--border-color);
        border-radius: 8px;
        display: flex;
        align-items: center;
        justify-content: center;
        color: var(--text-muted);
        font-size: 13px;
        transition: border-color 0.2s, background 0.2s;
    }

    .drop-placeholder:hover .dashed-box {
        border-color: var(--accent-color);
        background: rgba(var(--accent-color-rgb), 0.05);
    }

    .motion-toggle {
        display: flex;
        align-items: center;
        justify-content: space-between;
        background: var(--bg-color);
        padding: 12px 16px;
        border-radius: 8px;
        border: 1px solid var(--border-color);
    }

    .motion-toggle span {
        font-size: 13px;
        color: var(--text-color);
        font-weight: 500;
    }

    .toggle-group {
        display: flex;
        background: var(--bg-surface);
        border-radius: 6px;
        border: 1px solid var(--border-color);
        overflow: hidden;
    }

    .toggle-group button {
        background: transparent;
        border: none;
        padding: 6px 16px;
        font-size: 12px;
        color: var(--text-muted);
        cursor: pointer;
        transition: all 0.2s;
    }

    .toggle-group button.active {
        background: var(--accent-color);
        color: #000000;
    }

    .toggle-group button:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .vc-footer {
        display: flex;
        justify-content: stretch;
    }

    .start-stop-btn {
        flex: 1;
        padding: 12px;
        border-radius: 8px;
        border: none;
        background: var(--accent-color);
        color: #000000;
        font-weight: 600;
        font-size: 14px;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 8px;
        transition: all 0.2s;
    }

    .start-stop-btn:hover:not(:disabled) {
        opacity: 0.9;
    }

    .start-stop-btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
        background: var(--border-color);
        color: var(--text-muted);
    }

    .start-stop-btn.stop {
        background: var(--danger-color, #ff4444);
        color: #ffffff;
    }

    .empty-msg {
        color: var(--text-muted);
        font-size: 14px;
        text-align: center;
        padding: 20px;
    }

    /* Modal Styles */
    .modal-backdrop {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background: rgba(0, 0, 0, 0.6);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
        backdrop-filter: blur(4px);
    }

    .modal-content {
        background: var(--bg-surface);
        border: 1px solid var(--border-color);
        border-radius: 12px;
        padding: 24px;
        width: 90%;
        max-width: 500px;
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
    }

    .modal-content h3 {
        margin: 0 0 8px 0;
        font-size: 20px;
        color: var(--text-color);
    }

    .modal-content p {
        margin: 0 0 24px 0;
        color: var(--text-muted);
        font-size: 14px;
    }

    .modal-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
        gap: 16px;
        margin-bottom: 24px;
        max-height: 400px;
        overflow-y: auto;
    }

    .modal-card {
        background: var(--bg-color);
        border: 1px solid var(--border-color);
        border-radius: 8px;
        padding: 20px;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 12px;
        cursor: pointer;
        transition: all 0.2s;
    }

    .modal-card:hover {
        border-color: var(--accent-color);
        background: rgba(var(--accent-color-rgb), 0.05);
        transform: translateY(-2px);
    }

    .modal-card span {
        font-size: 13px;
        font-weight: 500;
        color: var(--text-color);
        text-align: center;
    }

    .modal-actions {
        display: flex;
        justify-content: flex-end;
    }

    .cancel-btn {
        background: transparent;
        border: 1px solid var(--border-color);
        color: var(--text-color);
        padding: 8px 16px;
        border-radius: 6px;
        cursor: pointer;
        font-weight: 500;
        transition: all 0.2s;
    }

    .cancel-btn:hover {
        background: var(--bg-color);
    }
</style>
