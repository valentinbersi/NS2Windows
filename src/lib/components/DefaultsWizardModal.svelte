<script lang="ts">
    import {
        ProfileKind,
        type Profile,
        DEFAULT_XBOX,
        DEFAULT_UPRIGHT_PS4,
        DEFAULT_FRONT_PS4,
        DEFAULT_NSO_GC_XBOX,
        DEFAULT_UPRIGHT_NSO_GC_PS4,
        DEFAULT_FRONT_NSO_GC_PS4,
        DEFAULT_SIDEWAYS_LEFT_JOY_CON_XBOX,
        DEFAULT_UPRIGHT_SIDEWAYS_LEFT_JOY_CON_PS4,
        DEFAULT_FRONT_SIDEWAYS_LEFT_JOY_CON_PS4,
        DEFAULT_SIDEWAYS_RIGHT_JOY_CON_XBOX,
        DEFAULT_UPRIGHT_SIDEWAYS_RIGHT_JOY_CON_PS4,
        DEFAULT_FRONT_SIDEWAYS_Right_JOY_CON_PS4
    } from "../types";

    export let currentKind: ProfileKind;
    export let onAccept: (profile: Profile) => void;
    export let onCancel: () => void;

    type PhysicalSetup = "standard" | "nso_gc" | "left_joycon" | "right_joycon";
    type Orientation = "upright" | "front";

    let physicalSetup: PhysicalSetup = "standard";
    let orientation: Orientation = "upright";

    function handleApply() {
        let selectedProfile: Profile;
        
        if (currentKind === ProfileKind.Xbox360) {
            switch (physicalSetup) {
                case "standard":
                    selectedProfile = DEFAULT_XBOX;
                    break;
                case "nso_gc":
                    selectedProfile = DEFAULT_NSO_GC_XBOX;
                    break;
                case "left_joycon":
                    selectedProfile = DEFAULT_SIDEWAYS_LEFT_JOY_CON_XBOX;
                    break;
                case "right_joycon":
                    selectedProfile = DEFAULT_SIDEWAYS_RIGHT_JOY_CON_XBOX;
                    break;
            }
        } else {
            // PS4
            switch (physicalSetup) {
                case "standard":
                    selectedProfile = orientation === "upright" ? DEFAULT_UPRIGHT_PS4 : DEFAULT_FRONT_PS4;
                    break;
                case "nso_gc":
                    selectedProfile = orientation === "upright" ? DEFAULT_UPRIGHT_NSO_GC_PS4 : DEFAULT_FRONT_NSO_GC_PS4;
                    break;
                case "left_joycon":
                    selectedProfile = orientation === "upright" ? DEFAULT_UPRIGHT_SIDEWAYS_LEFT_JOY_CON_PS4 : DEFAULT_FRONT_SIDEWAYS_LEFT_JOY_CON_PS4;
                    break;
                case "right_joycon":
                    selectedProfile = orientation === "upright" ? DEFAULT_UPRIGHT_SIDEWAYS_RIGHT_JOY_CON_PS4 : DEFAULT_FRONT_SIDEWAYS_Right_JOY_CON_PS4;
                    break;
            }
        }

        onAccept(selectedProfile);
    }
</script>

<div class="modal-backdrop" on:click={onCancel}>
    <div class="modal-content" on:click|stopPropagation>
        <h3>Set Default Mapping</h3>
        <p class="subtitle">Select your controller configuration to generate default inputs for {currentKind === ProfileKind.Ps4 ? "PlayStation 4" : "Xbox 360"}.</p>
        
        <div class="form-group">
            <label for="physicalSetup">Physical Controller Setup</label>
            <select id="physicalSetup" bind:value={physicalSetup}>
                <option value="standard">Pro Controller / Dual Joy-Cons</option>
                <option value="nso_gc">NSO GameCube Controller</option>
                <option value="left_joycon">Left Joy-Con (Sideways)</option>
                <option value="right_joycon">Right Joy-Con (Sideways)</option>
            </select>
        </div>

        {#if currentKind === ProfileKind.Ps4}
            <div class="form-group">
                <label for="orientation">Motion Orientation</label>
                <select id="orientation" bind:value={orientation}>
                    <option value="upright">Upright (Standard)</option>
                    <option value="front">Front-facing (Pointing Forward)</option>
                </select>
            </div>
        {/if}

        <div class="modal-actions">
            <button class="secondary" on:click={onCancel}>Cancel</button>
            <button class="primary" on:click={handleApply}>Apply Defaults</button>
        </div>
    </div>
</div>

<style>
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
        z-index: 100;
        backdrop-filter: blur(4px);
    }

    .modal-content {
        background: var(--bg-surface);
        border: 1px solid var(--border-color);
        padding: 24px;
        border-radius: 12px;
        width: 400px;
        max-width: 90vw;
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
    }

    h3 {
        margin: 0 0 8px 0;
        font-weight: 500;
    }

    .subtitle {
        font-size: 14px;
        color: var(--text-muted);
        margin: 0 0 20px 0;
        line-height: 1.4;
    }

    .form-group {
        display: flex;
        flex-direction: column;
        gap: 8px;
        margin-bottom: 20px;
    }

    .form-group label {
        font-size: 14px;
        font-weight: 500;
        color: var(--text-muted);
    }

    select {
        width: 100%;
        padding: 8px 12px;
        background: var(--bg-surface-hover);
        border: 1px solid var(--border-color);
        color: var(--text-color);
        border-radius: 6px;
    }

    .modal-actions {
        display: flex;
        justify-content: flex-end;
        gap: 12px;
        margin-top: 32px;
    }
</style>
