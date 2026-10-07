<script lang="ts">
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {load, type Store} from "@tauri-apps/plugin-store";
    import {DEFAULT_CEMUHOOK_ADDRESS, DEFAULT_CEMUHOOK_PORT, validateCemuHookSettings} from "../cemuhookSettings";

    let displayFrequency: number = 60;
    let emulationFrequency: number = 60;
    let cemuhookAddress = DEFAULT_CEMUHOOK_ADDRESS;
    let cemuhookPort: number = DEFAULT_CEMUHOOK_PORT;
    let errors: string[] = [];
    let applied = false;
    let loading = true;
    let saving = false;

    let store: Store | null = null;

    async function loadSettings() {
        loading = true;
        errors = [];
        applied = false;
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
            cemuhookAddress = await store.get<string>("cemuhook_address") ?? DEFAULT_CEMUHOOK_ADDRESS;
            cemuhookPort = await store.get<number>("cemuhook_port") ?? DEFAULT_CEMUHOOK_PORT;
        } catch (e) {
            console.error("Failed to load settings", e);
            errors = ["Failed to load settings: " + e];
        } finally {
            loading = false;
        }
    }

    onMount(() => {
        loadSettings();
    });

    async function applySettings() {
        if (saving) return;
        saving = true;
        errors = [];
        applied = false;
        try {
            const updates = [
                {label: "Input display frequency", valid: Number.isInteger(displayFrequency) && displayFrequency >= 1 && displayFrequency <= 65535,
                    apply: () => invoke("update_display_frequency", {newFrequency: displayFrequency})},
                {label: "Controller emulation frequency", valid: Number.isInteger(emulationFrequency) && emulationFrequency >= 1 && emulationFrequency <= 65535,
                    apply: () => invoke("update_emulation_frequency", {newFrequency: emulationFrequency})},
            ];
            // Apply separately: a blocked endpoint change must not prevent
            // changing emulation/display frequency, and vice versa.
            for (const update of updates) {
                if (!update.valid) { errors = [...errors, `${update.label} must be an integer between 1 and 65535.`]; continue; }
                try { await update.apply(); } catch (error) { errors = [...errors, `${update.label}: ${error}`]; }
            }
            const endpointError = validateCemuHookSettings(cemuhookAddress, cemuhookPort);
            if (endpointError) errors = [...errors, endpointError];
            else {
                try {
                    await invoke("update_cemuhook_settings", {address: cemuhookAddress.trim(), port: cemuhookPort});
                } catch (error) { errors = [...errors, `CemuHook: ${error}`]; }
            }
            applied = errors.length === 0;
        } catch (e) {
            console.error("Failed to apply settings", e);
            errors = [...errors, "Failed to apply settings: " + e];
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
        cemuhookAddress = DEFAULT_CEMUHOOK_ADDRESS;
        cemuhookPort = DEFAULT_CEMUHOOK_PORT;
        errors = [];
        applied = false;
    }

    function handleDisplayFrequencyInput(e: Event) {
        displayFrequency = (e.target as HTMLInputElement).valueAsNumber;
    }

    function handleEmulationFrequencyInput(e: Event) {
        emulationFrequency = (e.target as HTMLInputElement).valueAsNumber;
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
                        max="65535"
                        disabled={saving}
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
                        max="65535"
                        disabled={saving}
                        step="1"
                        value={emulationFrequency}
                        on:input={handleEmulationFrequencyInput}
                />
                <span class="help-text">How often the emulated controller sends inputs to the system.</span>
            </div>

            <div class="form-group">
                <label for="cemuhook-address">CemuHook Bind Address</label>
                <input id="cemuhook-address" type="text" bind:value={cemuhookAddress} disabled={saving} spellcheck="false" />
                <span class="help-text">Use 127.0.0.1 for emulators on this PC, or a local network address for LAN clients. Stop all CemuHook controllers before changing the address or port.</span>
            </div>
            <div class="form-group">
                <label for="cemuhook-port">CemuHook Port</label>
                <input id="cemuhook-port" type="number" min="1" max="65535" step="1" value={cemuhookPort}
                    on:input={(event) => cemuhookPort = event.currentTarget.valueAsNumber} disabled={saving} />
                <span class="help-text">Set the same address and port in your emulator’s CemuHook/DSU client. Default port: 26760.</span>
            </div>
            {#if errors.length > 0}
                <div class="settings-errors" role="alert">
                    {#each errors as error}<p>{error}</p>{/each}
                    <p>Other valid settings were applied.</p>
                </div>
            {:else if applied}
                <p role="status">Settings applied.</p>
            {/if}
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
    .settings-errors { color: var(--error-color, #f87171); font-size: 14px; }
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
