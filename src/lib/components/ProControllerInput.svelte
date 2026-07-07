<script lang="ts">
    import type {NsInput} from "../types";
    import MotionInputBars from "./MotionInputBars.svelte";

    export let inputs: Partial<Record<NsInput, number>> = {};
    $: isActive = (input: NsInput) => (inputs[input] || 0) > 0.5;

    $: lStickX = ((inputs.LeftXPlus || 0) + -(inputs.LeftXMinus || 0)) * 8;
    $: lStickY = (-(inputs.LeftYMinus || 0) + (inputs.LeftYPlus || 0)) * 8;
    $: rStickX = ((inputs.RightXPlus || 0) + -(inputs.RightXMinus || 0)) * 8;
    $: rStickY = (-(inputs.RightYMinus || 0) + (inputs.RightYPlus || 0)) * 8;
</script>
<div class="pro-controller-container">
    <svg height="150" style="max-height: 200px;" viewBox="0 0 200 150" width="200">
        <!-- Body roughly like Pro Controller -->
        <path d="M 40 20 Q 100 20 160 20 Q 180 20 190 50 Q 200 90 170 130 Q 150 150 140 120 Q 130 90 100 90 Q 70 90 60 120 Q 50 150 30 130 Q 0 90 10 50 Q 20 20 40 20 Z"
              fill="var(--bg-surface-hover)" stroke="var(--border-color)" stroke-width="2"/>

        <!-- D-pad -->
        <rect fill={isActive('Right') || isActive('Left') ? 'var(--accent-color)' : '#444'} height="8" rx="2" transform="rotate(90 65 89)" width="20"
              x="55" y="85"/>
        <rect fill={isActive('Up') || isActive('Down') ? 'var(--accent-color)' : '#444'} height="8" rx="2" width="20" x="55"
              y="85"/>
        <circle cx="65" cy="80" fill={isActive('Up') ? 'var(--accent-color)' : 'transparent'} r="4"/>
        <circle cx="65" cy="98" fill={isActive('Down') ? 'var(--accent-color)' : 'transparent'} r="4"/>
        <circle cx="56" cy="89" fill={isActive('Left') ? 'var(--accent-color)' : 'transparent'} r="4"/>
        <circle cx="74" cy="89" fill={isActive('Right') ? 'var(--accent-color)' : 'transparent'} r="4"/>

        <!-- ABXY Buttons -->
        <circle cx="150" cy="55" fill="none" r="20" stroke="rgba(255,255,255,0.05)"/>
        <circle cx="150" cy="40" fill={isActive('X') ? 'var(--accent-color)' : '#444'} r="5"/>
        <circle cx="150" cy="70" fill={isActive('B') ? 'var(--accent-color)' : '#444'} r="5"/>
        <circle cx="135" cy="55" fill={isActive('Y') ? 'var(--accent-color)' : '#444'} r="5"/>
        <circle cx="165" cy="55" fill={isActive('A') ? 'var(--accent-color)' : '#444'} r="5"/>

        <!-- L Stick -->
        <circle cx="55" cy="55" fill="#111" r="14"/>
        <circle cx={55 + lStickX} cy={55 + lStickY} fill={isActive('LeftXPlus') || isActive('LeftXMinus') || isActive('LeftYPlus') || isActive('LeftYMinus') ? 'var(--accent-color)' : '#444'}
                r="10"/>

        <!-- R Stick -->
        <circle cx="130" cy="100" fill="#111" r="14"/>
        <circle cx={130 + rStickX} cy={100 + rStickY} fill={isActive('RightXPlus') || isActive('RightXMinus') || isActive('RightYPlus') || isActive('RightYMinus') ? 'var(--accent-color)' : '#444'}
                r="10"/>

        <!-- Minus & Plus -->
        <rect fill={isActive('Minus') ? 'var(--accent-color)' : '#444'} height="3" rx="1" width="10" x="75" y="35"/>
        <path d="M 115 36 L 125 36 M 120 31 L 120 41" stroke={isActive('Plus') ? 'var(--accent-color)' : '#444'}
              stroke-linecap="round" stroke-width="3"/>

        <!-- Home & Capture -->
        <rect fill={isActive('Capture') ? 'var(--accent-color)' : '#444'} height="8" rx="1" width="8" x="85" y="60"/>
        <circle cx="110" cy="64" fill={isActive('Home') ? 'var(--accent-color)' : '#444'} r="5"/>

        <!-- Shoulders -->
        <path d="M 30 30 Q 40 10 70 10 L 70 15 Q 45 15 35 30 Z" fill={isActive('L') ? 'var(--accent-color)' : '#444'}/>
        <path d="M 25 25 Q 40 0 75 0 L 75 5 Q 45 5 30 25 Z" fill={isActive('Zl') ? 'var(--accent-color)' : '#444'}/>

        <path d="M 170 30 Q 160 10 130 10 L 130 15 Q 155 15 165 30 Z"
              fill={isActive('R') ? 'var(--accent-color)' : '#444'}/>
        <path d="M 175 25 Q 160 0 125 0 L 125 5 Q 155 5 170 25 Z" fill={isActive('Zr') ? 'var(--accent-color)' : '#444'}/>
    </svg>

    <MotionInputBars {inputs}/>
</div>

<style>
    .pro-controller-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 8px;
        width: 100%;
    }
</style>
