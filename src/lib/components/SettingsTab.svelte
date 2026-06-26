<script lang="ts">
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {load, Store} from "@tauri-apps/plugin-store";

    let displayFrequency: number = 60;
    let emulationFrequency: number = 60;
    let loading = true;
    let saving = false;

    let store: Store | null = null;

    async function loadSettings() {
        loading = true;
        try {
            if (!store) {
                store = await load("settings.json");
            }
            const df = await store?.get<number>("display_frequency");
            const ef = await store?.get<number>("emulation_frequency");

            if (df !== null && df !== undefined) {
                displayFrequency = df;
            }
            if (ef !== null && ef !== undefined) {
                emulationFrequency = ef;
            }
        } catch (e) {
            console.error("Failed to load settings", e);
        } finally {
            loading = false;
        }
    }

    onMount(() => {
        loadSettings();
    });

    async function applySettings() {
        saving = true;
        try {
            await invoke("update_display_frequency", {newFrequency: displayFrequency});
            await invoke("update_emulation_frequency", {newFrequency: emulationFrequency});
        } catch (e) {
            console.error("Failed to apply settings", e);
            alert("Failed to apply settings: " + e);
        } finally {
            saving = false;
        }
    }

    function restoreCurrent() {
        loadSettings();
    }

    function restoreDefault() {
        displayFrequency = 60;
        emulationFrequency = 60;
    }

    function handleDisplayFrequencyInput(e: Event) {
        const val = parseInt((e.target as HTMLInputElement).value);
        if (!isNaN(val)) displayFrequency = val;
    }

    function handleEmulationFrequencyInput(e: Event) {
        const val = parseInt((e.target as HTMLInputElement).value);
        if (!isNaN(val)) emulationFrequency = val;
    }

</script>

<div class="tab-container">
    <div class="header">
        <h2>Settings</h2>
        <p class="subtitle">Configure application-wide settings</p>
    </div>

    {#if loading}
        <div class="loading-state">Loading settings...</div>
    {:else}
        <div class="settings-content">
            <div class="form-group">
                <label for="display-frequency">Input Display Frequency (Hz)</label>
                <input
                        id="display-frequency"
                        type="number"
                        min="1"
                        step="1"
                        value={displayFrequency}
                        on:input={handleDisplayFrequencyInput}
                />
                <span class="help-text">How often the input state is updated in the UI.</span>
            </div>

            <div class="form-group">
                <label for="emulation-frequency">Controller Emulation Frequency (Hz)</label>
                <input
                        id="emulation-frequency"
                        type="number"
                        min="1"
                        step="1"
                        value={emulationFrequency}
                        on:input={handleEmulationFrequencyInput}
                />
                <span class="help-text">How often the emulated controller sends inputs to the system.</span>
            </div>

            <div class="actions">
                <button class="secondary" on:click={restoreDefault} disabled={saving}>Restore to Default</button>
                <button class="secondary" on:click={restoreCurrent} disabled={saving}>Restore to Current</button>
                <button class="primary" on:click={applySettings} disabled={saving}>
                    {saving ? 'Applying...' : 'Apply Changes'}
                </button>
            </div>
        </div>
    {/if}
</div>

<style>
    .tab-container {
        padding: 24px 32px;
        height: 100%;
        display: flex;
        flex-direction: column;
        overflow-y: auto;
    }

    .header {
        margin-bottom: 32px;
    }

    .header h2 {
        margin: 0;
        font-size: 24px;
        font-weight: 600;
        color: var(--text-color);
    }

    .subtitle {
        margin: 8px 0 0 0;
        color: var(--text-muted);
        font-size: 14px;
    }

    .loading-state {
        display: flex;
        justify-content: center;
        align-items: center;
        flex: 1;
        color: var(--text-muted);
    }

    .settings-content {
        display: flex;
        flex-direction: column;
        gap: 24px;
        max-width: 600px;
    }

    .form-group {
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .form-group label {
        font-weight: 500;
        color: var(--text-color);
    }

    .form-group input {
        padding: 10px 12px;
        border-radius: 6px;
        border: 1px solid var(--border-color);
        background: var(--bg-surface);
        color: var(--text-color);
        font-size: 16px;
        transition: border-color 0.2s;
    }

    .form-group input:focus {
        outline: none;
        border-color: var(--accent-color);
    }

    .help-text {
        font-size: 13px;
        color: var(--text-muted);
    }

    .actions {
        display: flex;
        gap: 12px;
        margin-top: 16px;
        padding-top: 24px;
        border-top: 1px solid var(--border-color);
    }

    button {
        padding: 10px 16px;
        border-radius: 6px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s;
        border: none;
        font-size: 14px;
    }

    button:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    button.primary {
        background: var(--accent-color);
        color: black;
    }

    button.primary:hover:not(:disabled) {
        filter: brightness(1.1);
    }

    button.secondary {
        background: var(--bg-surface);
        border: 1px solid var(--border-color);
        color: var(--text-color);
    }

    button.secondary:hover:not(:disabled) {
        background: var(--bg-surface-hover);
        border-color: var(--accent-color);
    }
</style>
