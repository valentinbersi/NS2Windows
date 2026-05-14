<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { ProfileKind, NS_INPUT_LABELS, PS4_OUTPUT_LABELS, XBOX360_OUTPUT_LABELS } from "../types";
    import type { Profile, NsInput, Output, Condition } from "../types";

    export let profileName: string | null = null;
    export let onBack: () => void;

    let profile: Profile = {
        profile_name: "",
        profile_kind: ProfileKind.Ps4,
        outputs: {}
    };

    let loading = true;

    // Derived list of outputs based on kind
    $: activeOutputs = Object.entries(
        profile.profile_kind === ProfileKind.Ps4 ? PS4_OUTPUT_LABELS : XBOX360_OUTPUT_LABELS
    )
        .filter(([_, label]) => label !== null)
        .map(([key, label]) => ({ key: key as Output, label: label as string }));

    const allNsInputs = Object.entries(NS_INPUT_LABELS).map(([key, label]) => ({
        key: key as NsInput,
        label,
    }));

    onMount(async () => {
        if (profileName) {
            try {
                const existing = await invoke<Profile | null>("find_profile_by_name", { name: profileName });
                if (existing) {
                    // Rust serializes enums without wrapping if configured, but let's assume it matches our interface.
                    profile = {
                        profile_name: existing.profile_name,
                        profile_kind: existing.profile_kind,
                        // Ensure outputs is at least an empty object
                        outputs: existing.outputs || {}
                    };
                }
            } catch (e) {
                console.error("Failed to load profile", e);
            }
        }
        loading = false;
    });

    async function handleSave() {
        if (!profile.profile_name.trim()) {
            alert("Profile name cannot be empty");
            return;
        }
        
        try {
            await invoke("save_profile", { profile });
            onBack();
        } catch (e) {
            console.error("Failed to save profile", e);
            alert("Failed to save profile");
        }
    }

    function handleOutputChange(output: Output, value: string) {
        if (value === "None") {
            const newOutputs = { ...profile.outputs };
            delete newOutputs[output];
            profile.outputs = newOutputs;
        } else {
            profile.outputs = {
                ...profile.outputs,
                [output]: { Value: value as NsInput }
            };
        }
    }
</script>

<div class="editor-container">
    {#if loading}
        <div class="loading">Loading...</div>
    {:else}
        <div class="header">
            <div class="header-title">
                <button class="back-btn" on:click={onBack}>← Back</button>
                <h2>{profileName ? "Edit Profile" : "New Profile"}</h2>
            </div>
            <div class="header-actions">
                <button on:click={onBack}>Cancel</button>
                <button class="primary" on:click={handleSave}>Save Profile</button>
            </div>
        </div>

        <div class="form-section">
            <div class="input-group">
                <label for="profileName">Profile Name</label>
                <input 
                    id="profileName" 
                    type="text" 
                    bind:value={profile.profile_name} 
                    placeholder="e.g. Smash Bros layout"
                    disabled={profileName !== null} 
                />
                {#if profileName !== null}
                    <small class="hint">Profile name cannot be changed after creation.</small>
                {/if}
            </div>

            <div class="input-group">
                <label for="profileKind">Emulated Controller Type</label>
                <select id="profileKind" bind:value={profile.profile_kind}>
                    <option value={ProfileKind.Ps4}>PlayStation 4</option>
                    <option value={ProfileKind.Xbox360}>Xbox 360</option>
                </select>
            </div>
        </div>

        <div class="mapping-section">
            <h3>Input Mapping</h3>
            <p class="subtitle">Map Nintendo Switch inputs to virtual {profile.profile_kind} outputs.</p>

            <div class="table-container">
                <table>
                    <thead>
                        <tr>
                            <th>Virtual Output ({profile.profile_kind})</th>
                            <th>Nintendo Switch Input</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each activeOutputs as { key, label }}
                            <tr>
                                <td class="output-cell">{label}</td>
                                <td>
                                    <select 
                                        value={profile.outputs[key]?.Value || "None"}
                                        on:change={(e) => handleOutputChange(key, e.currentTarget.value)}
                                    >
                                        <option value="None">-- Unmapped --</option>
                                        {#each allNsInputs as input}
                                            <option value={input.key}>{input.label}</option>
                                        {/each}
                                    </select>
                                </td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
        </div>
    {/if}
</div>

<style>
    .editor-container {
        display: flex;
        flex-direction: column;
        height: 100%;
        overflow: hidden;
    }

    .loading {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100%;
        color: var(--text-muted);
    }

    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 20px 32px;
        border-bottom: 1px solid var(--border-color);
        background: var(--bg-surface);
    }

    .header-title {
        display: flex;
        align-items: center;
        gap: 16px;
    }

    .header-title h2 {
        margin: 0;
        font-weight: 500;
    }

    .back-btn {
        background: transparent;
        border: none;
        color: var(--text-muted);
        padding: 4px 8px;
    }

    .back-btn:hover {
        color: var(--text-color);
    }

    .header-actions {
        display: flex;
        gap: 12px;
    }

    .form-section {
        padding: 24px 32px;
        display: flex;
        gap: 32px;
        border-bottom: 1px solid var(--border-color);
    }

    .input-group {
        display: flex;
        flex-direction: column;
        gap: 8px;
        flex: 1;
        max-width: 400px;
    }

    .input-group label {
        font-size: 14px;
        color: var(--text-muted);
        font-weight: 500;
    }

    .hint {
        color: var(--text-muted);
        font-size: 12px;
    }

    .mapping-section {
        display: flex;
        flex-direction: column;
        flex: 1;
        padding: 24px 32px;
        overflow: hidden;
    }

    .mapping-section h3 {
        margin: 0 0 4px 0;
        font-weight: 500;
    }

    .subtitle {
        color: var(--text-muted);
        margin: 0 0 20px 0;
        font-size: 14px;
    }

    .table-container {
        flex: 1;
        overflow-y: auto;
        border: 1px solid var(--border-color);
        border-radius: 8px;
        background: var(--bg-surface);
    }

    table {
        width: 100%;
        border-collapse: collapse;
        text-align: left;
    }

    th {
        position: sticky;
        top: 0;
        background: var(--bg-surface-hover);
        padding: 12px 16px;
        font-weight: 500;
        color: var(--text-muted);
        border-bottom: 1px solid var(--border-color);
        z-index: 1;
    }

    td {
        padding: 10px 16px;
        border-bottom: 1px solid var(--border-color);
    }

    tr:last-child td {
        border-bottom: none;
    }

    tr:hover td {
        background: rgba(255, 255, 255, 0.02);
    }

    .output-cell {
        font-weight: 500;
        width: 40%;
    }

    select {
        width: 100%;
        max-width: 300px;
    }
</style>
