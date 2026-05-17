<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { ControllerKind, ProfileKind, CONTROLLER_KIND_LABELS } from "../types";
    import type { Connection, Profile } from "../types";

    import JoyConLeftIcon from "./icons/JoyConLeftIcon.svelte";
    import JoyConRightIcon from "./icons/JoyConRightIcon.svelte";
    import DualJoyConsIcon from "./icons/DualJoyConsIcon.svelte";
    import ProControllerIcon from "./icons/ProControllerIcon.svelte";
    import Ps4Icon from "./icons/Ps4Icon.svelte";
    import Xbox360Icon from "./icons/Xbox360Icon.svelte";

    let connections: Connection[] = [];
    let profiles: Profile[] = [];
    let loading = true;

    // The user-declared pairs. Each object maps a chosen connection to a profile.
    let associations: { connectionId: string, profileName: string }[] = [];

    // State for the "Add Association" form
    let selectedConnectionId = "";
    let selectedProfileName = "";

    // Application state
    let isRunning = false;

    async function loadData() {
        try {
            const [conns, profileNames] = await Promise.all([
                invoke<Connection[]>("get_connections"),
                invoke<string[]>("profile_names")
            ]);

            connections = conns;

            // Fetch the full profile objects to get their ProfileKind
            const loadedProfiles: Profile[] = [];
            for (const name of profileNames) {
                const p = await invoke<Profile | null>("find_profile_by_name", { name });
                if (p) loadedProfiles.push(p);
            }
            profiles = loadedProfiles;

        } catch (e) {
            console.error("Failed to load data", e);
        } finally {
            loading = false;
        }
    }

    onMount(() => {
        loadData();
    });

    // Connections that are NOT currently used in an association
    $: availableConnections = connections.filter(
        c => !associations.some(a => a.connectionId === c.id)
    );

    function addAssociation() {
        if (!selectedConnectionId || !selectedProfileName) return;

        associations = [...associations, { 
            connectionId: selectedConnectionId, 
            profileName: selectedProfileName 
        }];

        selectedConnectionId = "";
        selectedProfileName = "";
    }

    function removeAssociation(connectionId: string) {
        associations = associations.filter(a => a.connectionId !== connectionId);
    }

    async function startControllers() {
        if (associations.length === 0) return;

        // Backend expects: Vec<(String, Uuid)>
        // In JS we pass: [[profileName, connectionId], ...]
        const payload = associations.map(a => [a.profileName, a.connectionId]);

        try {
            await invoke("start_controllers", { controllers: payload });
            isRunning = true;
        } catch (e) {
            console.error("Failed to start controllers", e);
            alert("Failed to start controllers: " + e);
        }
    }

    function getConnectionInfo(id: string) {
        return connections.find(c => c.id === id);
    }

    function getProfileInfo(name: string) {
        return profiles.find(p => p.profile_name === name);
    }

    function getControllerIcon(kind: ControllerKind) {
        switch (kind) {
            case ControllerKind.LeftJoyCon: return JoyConLeftIcon;
            case ControllerKind.RightJoyCon: return JoyConRightIcon;
            case ControllerKind.DualJoyCons: return DualJoyConsIcon;
            case ControllerKind.ProNsoGcController: return ProControllerIcon;
            default: return ProControllerIcon;
        }
    }

    function getProfileIcon(kind: ProfileKind) {
        return kind === ProfileKind.Ps4 ? Ps4Icon : Xbox360Icon;
    }
</script>

<div class="tab-container">
    {#if loading}
        <div class="empty-state">Loading data...</div>
    {:else if isRunning}
        <!-- Running State View -->
        <div class="running-state">
            <div class="spinner"></div>
            <h2>Controllers are Running</h2>
            <p>Your emulated controllers are now active in the background.</p>
            
            <div class="running-associations">
                {#each associations as assoc}
                    {@const conn = getConnectionInfo(assoc.connectionId)}
                    {@const prof = getProfileInfo(assoc.profileName)}
                    {#if conn && prof}
                        <div class="running-card">
                            <svelte:component this={getControllerIcon(conn.controller_kind)} width="24" height="24" />
                            <span class="arrow">→</span>
                            <svelte:component this={getProfileIcon(prof.profile_kind)} width="24" height="24" />
                            <span class="profile-label">{prof.profile_name}</span>
                        </div>
                    {/if}
                {/each}
            </div>
            
            <p class="hint">Restart the application to stop the emulation.</p>
        </div>
    {:else}
        <!-- Configuration State View -->
        <div class="toolbar">
            <h2>Virtual Controllers</h2>
            <button 
                class="primary" 
                disabled={associations.length === 0} 
                on:click={startControllers}
            >
                ▶ Start Emulation
            </button>
        </div>
        
        <p class="subtitle">Associate your connected devices with your custom profiles.</p>

        <!-- Form for adding association -->
        <div class="form-section">
            <div class="input-group">
                <label for="connectionSelect">Connected Device</label>
                <select id="connectionSelect" bind:value={selectedConnectionId}>
                    <option value="" disabled>Select a connection...</option>
                    {#each availableConnections as conn (conn.id)}
                        <option value={conn.id}>
                            {CONTROLLER_KIND_LABELS[conn.controller_kind]} ({conn.id.split('-')[0]})
                        </option>
                    {/each}
                </select>
            </div>

            <div class="input-group">
                <label for="profileSelect">Target Profile</label>
                <select id="profileSelect" bind:value={selectedProfileName}>
                    <option value="" disabled>Select a profile...</option>
                    {#each profiles as profile}
                        <option value={profile.profile_name}>
                            {profile.profile_name} ({profile.profile_kind})
                        </option>
                    {/each}
                </select>
            </div>

            <button 
                class="add-btn" 
                disabled={!selectedConnectionId || !selectedProfileName}
                on:click={addAssociation}
            >
                Add Association
            </button>
        </div>

        <!-- List of Associations -->
        {#if associations.length > 0}
            <div class="associations-list">
                <h3>Configured Emulators</h3>
                {#each associations as assoc (assoc.connectionId)}
                    {@const conn = getConnectionInfo(assoc.connectionId)}
                    {@const prof = getProfileInfo(assoc.profileName)}
                    
                    {#if conn && prof}
                        <div class="association-card">
                            <div class="pairing-info">
                                <div class="device-col">
                                    <svelte:component this={getControllerIcon(conn.controller_kind)} width="32" height="32" />
                                    <div class="col-text">
                                        <span class="label">Source</span>
                                        <span class="value">{CONTROLLER_KIND_LABELS[conn.controller_kind]}</span>
                                    </div>
                                </div>
                                
                                <div class="arrow-col">
                                    <span class="arrow">→</span>
                                </div>

                                <div class="device-col">
                                    <svelte:component this={getProfileIcon(prof.profile_kind)} width="32" height="32" />
                                    <div class="col-text">
                                        <span class="label">Target ({prof.profile_kind})</span>
                                        <span class="value">{prof.profile_name}</span>
                                    </div>
                                </div>
                            </div>
                            
                            <button 
                                class="action-btn delete-btn" 
                                title="Remove Association"
                                on:click={() => removeAssociation(assoc.connectionId)}
                            >
                                🗑
                            </button>
                        </div>
                    {/if}
                {/each}
            </div>
        {:else}
            <div class="empty-state">
                <p>No associations declared yet. Select a device and a profile above to get started.</p>
            </div>
        {/if}
    {/if}
</div>

<style>
    .tab-container {
        display: flex;
        flex-direction: column;
        height: 100%;
        padding: 24px 32px;
        gap: 20px;
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

    .subtitle {
        color: var(--text-muted);
        margin: -10px 0 0 0;
        font-size: 14px;
    }

    .form-section {
        display: flex;
        gap: 16px;
        align-items: flex-end;
        padding: 20px;
        background: var(--bg-surface);
        border: 1px solid var(--border-color);
        border-radius: 8px;
    }

    .input-group {
        display: flex;
        flex-direction: column;
        gap: 8px;
        flex: 1;
    }

    .input-group label {
        font-size: 13px;
        color: var(--text-muted);
        font-weight: 500;
    }

    .add-btn {
        padding: 8px 16px;
        white-space: nowrap;
    }

    .add-btn:not(:disabled) {
        background: var(--text-color);
        color: var(--bg-color);
        border-color: var(--text-color);
    }

    .associations-list {
        flex: 1;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        gap: 12px;
    }

    .associations-list h3 {
        margin: 0 0 8px 0;
        font-size: 16px;
        font-weight: 500;
        color: var(--text-muted);
    }

    .association-card {
        display: flex;
        justify-content: space-between;
        align-items: center;
        background: var(--bg-color);
        border: 1px solid var(--border-color);
        border-radius: 8px;
        padding: 16px 20px;
    }

    .pairing-info {
        display: flex;
        align-items: center;
        gap: 32px;
    }

    .device-col {
        display: flex;
        align-items: center;
        gap: 16px;
        width: 200px;
    }

    .col-text {
        display: flex;
        flex-direction: column;
        gap: 2px;
    }

    .col-text .label {
        font-size: 12px;
        color: var(--text-muted);
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .col-text .value {
        font-size: 15px;
        font-weight: 500;
        color: var(--text-color);
    }

    .arrow-col {
        color: var(--text-muted);
        font-size: 24px;
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

    .empty-state {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        color: var(--text-muted);
        text-align: center;
    }

    /* Running State */
    .running-state {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 16px;
        text-align: center;
    }

    .running-state h2 {
        margin: 0;
        color: var(--text-color);
    }

    .running-state p {
        color: var(--text-muted);
        margin: 0;
    }

    .running-associations {
        margin-top: 24px;
        display: flex;
        flex-direction: column;
        gap: 12px;
    }

    .running-card {
        display: flex;
        align-items: center;
        gap: 16px;
        padding: 12px 24px;
        background: var(--bg-surface);
        border: 1px solid var(--border-color);
        border-radius: 24px;
        color: var(--text-muted);
    }

    .running-card .profile-label {
        color: var(--text-color);
        font-weight: 500;
    }

    .hint {
        margin-top: 48px !important;
        font-size: 12px;
        opacity: 0.7;
    }

    .spinner {
        width: 60px;
        height: 60px;
        border: 4px solid var(--border-color);
        border-top-color: var(--accent-color);
        border-radius: 50%;
        animation: spin 1s linear infinite;
        margin-bottom: 16px;
    }

    @keyframes spin {
        to { transform: rotate(360deg); }
    }
</style>
