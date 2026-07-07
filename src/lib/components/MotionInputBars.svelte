<script lang="ts">
    import type {NsInput} from "../types";

    export let inputs: Partial<Record<NsInput, number>> = {};

    const MOTION_AXES: { input: NsInput; label: string }[] = [
        {input: "AccelUp", label: "Accel Up"},
        {input: "AccelDown", label: "Accel Down"},
        {input: "AccelLeft", label: "Accel Left"},
        {input: "AccelRight", label: "Accel Right"},
        {input: "AccelForward", label: "Accel Forward"},
        {input: "AccelBackward", label: "Accel Backward"},
        {input: "GyroPitchUp", label: "Pitch Up"},
        {input: "GyroPitchDown", label: "Pitch Down"},
        {input: "GyroYawRight", label: "Yaw Right"},
        {input: "GyroYawLeft", label: "Yaw Left"},
        {input: "GyroRollLeft", label: "Roll Left"},
        {input: "GyroRollRight", label: "Roll Right"},
    ];

    $: motionAxes = MOTION_AXES.map((axis) => {
        const rawValue = inputs[axis.input] ?? 0;
        const value = Number.isFinite(rawValue) ? rawValue : 0;

        return {
            ...axis,
            value,
            fillWidth: Math.max(0, Math.min(1, value)) * 100,
            displayValue: value.toFixed(2),
        };
    });
</script>

<div aria-label="Motion inputs" class="motion-inputs">
    {#each motionAxes as axis}
        <div class="motion-axis">
            <div class="motion-label">{axis.label}</div>
            <div
                    class="motion-bar"
                    role="progressbar"
                    aria-label={axis.label}
                    aria-valuemin="0"
                    aria-valuemax="1"
                    aria-valuenow={Math.max(0, Math.min(1, axis.value))}
            >
                <div class="motion-bar-fill" style={`width: ${axis.fillWidth}%`}></div>
                <div class="motion-value"><span>{axis.displayValue}</span></div>
            </div>
        </div>
    {/each}
</div>

<style>
    .motion-inputs {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
        gap: 8px 12px;
        justify-content: start;
        width: 100%;
        max-width: 520px;
        margin-top: 12px;
    }

    .motion-axis {
        display: grid;
        grid-template-columns: 104px minmax(0, 1fr);
        align-items: center;
        gap: 8px;
        min-width: 0;
    }

    .motion-label {
        color: var(--text-muted);
        font-size: 10px;
        line-height: 1.1;
        text-transform: uppercase;
        text-align: left;
        white-space: nowrap;
    }

    .motion-bar {
        position: relative;
        height: 18px;
        min-width: 0;
        overflow: hidden;
        border: 1px solid var(--border-color);
        border-radius: 4px;
        background: #111;
    }

    .motion-bar-fill {
        height: 100%;
        background: var(--accent-color);
        opacity: 0.85;
        transition: width 80ms linear;
    }

    .motion-value {
        position: absolute;
        inset: 0;
        display: flex;
        align-items: center;
        justify-content: center;
        color: var(--text-color);
        font-size: 10px;
        font-variant-numeric: tabular-nums;
        line-height: 1;
        pointer-events: none;
    }

    .motion-value span {
        min-width: 34px;
        padding: 2px 5px;
        border: 1px solid rgba(255, 255, 255, 0.2);
        border-radius: 3px;
        background: rgba(0, 0, 0, 0.78);
        color: #fff;
        text-align: center;
    }
</style>
