<script lang="ts">
    import type {NsInput} from "../types";

    export let inputs: Partial<Record<NsInput, number>> = {};
    $: isActive = (input: NsInput) => (inputs[input] || 0) > 0.5;

    $: lStickX = ((inputs.LeftXPlus || 0) + -(inputs.LeftXMinus || 0)) * 8;
    $: lStickY = -((-(inputs.LeftYMinus || 0) + (inputs.LeftYPlus || 0)) * 8);
    $: cStickX = ((inputs.RightXPlus || 0) + -(inputs.RightXMinus || 0)) * 8;
    $: cStickY = -((-(inputs.RightYMinus || 0) + (inputs.RightYPlus || 0)) * 8);

    $: lPressure = Math.max(0, Math.min(1, inputs.LTrigger || 0));
    $: rPressure = Math.max(0, Math.min(1, inputs.RTrigger || 0));
</script>

<div class="gc-container">
    <div class="view-label">Top</div>
    <svg class="top-view" height="70" viewBox="0 0 200 70" width="200">
        <!-- Top Body shape -->
        <path d="M 30 70 L 30 30 Q 30 10 50 10 L 150 10 Q 170 10 170 30 L 170 70 Z" fill="var(--bg-surface-hover)"
              stroke="var(--border-color)" stroke-width="2"/>

        <!-- ZL Bumper (Left) -->
        <rect fill={isActive('Zl') ? 'var(--accent-color)' : '#444'} height="8" rx="2" width="25" x="35" y="52"/>
        <!-- L Trigger (Analog + Digital click) -->
        <rect fill="#111" height="20" rx="3" width="30" x="35" y="25"/>
        <rect fill="var(--accent-color)" height={18 * lPressure} opacity={isActive('L') ? 1 : 0.5} rx="2" width="28"
              x="36" y={26 + 18 * (1 - lPressure)}/>
        <rect fill="none" height="20" rx="3" stroke="rgba(255,255,255,0.1)" stroke-width="1" width="30" x="35" y="25"/>

        <!-- Z Bumper (Right) -->
        <rect fill={isActive('Zr') ? 'var(--accent-color)' : '#444'} height="8" rx="2" width="25" x="140" y="52"/>
        <!-- R Trigger (Analog + Digital click) -->
        <rect fill="#111" height="20" rx="3" width="30" x="135" y="25"/>
        <rect fill="var(--accent-color)" height={18 * rPressure} opacity={isActive('R') ? 1 : 0.5} rx="2" width="28"
              x="136"
              y={26 + 18 * (1 - rPressure)}/>
        <rect fill="none" height="20" rx="3" stroke="rgba(255,255,255,0.1)" stroke-width="1" width="30" x="135" y="25"/>

        <!-- System Buttons (NW, NE, SE grid) -->
        <!-- Capture (NW) -->
        <rect fill={isActive('Capture') ? 'var(--accent-color)' : '#444'} height="10" rx="2" width="10" x="80" y="32"/>
        <!-- Home (NE) -->
        <circle cx="115" cy="37" fill={isActive('Home') ? 'var(--accent-color)' : '#444'} r="5"/>
        <!-- Chat (SE) -->
        <rect fill={isActive('Chat') ? 'var(--accent-color)' : '#444'} height="10" rx="2" width="10" x="110" y="46"/>
    </svg>

    <div class="view-label">Front</div>
    <svg class="front-view" height="150" viewBox="0 0 200 150" width="200">
        <!-- GC Body -->
        <path d="M 40 20 Q 100 20 160 20 Q 180 20 190 50 Q 200 90 170 130 Q 150 150 140 120 Q 130 90 100 90 Q 70 90 60 120 Q 50 150 30 130 Q 0 90 10 50 Q 20 20 40 20 Z"
              fill="var(--bg-surface-hover)" stroke="var(--border-color)" stroke-width="2"/>

        <!-- D-pad -->
        <g transform="translate(65, 89)">
            <!-- Up -->
            <polygon fill={isActive('Up') ? 'var(--accent-color)' : '#444'} points="-4,-4 4,-4 4,-14 -4,-14"/>
            <!-- Down -->
            <polygon fill={isActive('Down') ? 'var(--accent-color)' : '#444'} points="-4,4 4,4 4,14 -4,14"/>
            <!-- Left -->
            <polygon fill={isActive('Left') ? 'var(--accent-color)' : '#444'} points="-4,-4 -4,4 -14,4 -14,-4"/>
            <!-- Right -->
            <polygon fill={isActive('Right') ? 'var(--accent-color)' : '#444'} points="4,-4 4,4 14,4 14,-4"/>
            <!-- Center piece (always dark) -->
            <rect fill="#444" height="8" width="8" x="-4" y="-4"/>
        </g>

        <!-- L Stick -->
        <circle cx="55" cy="55" fill="#111" r="14"/>
        <circle cx={55 + lStickX} cy={55 + lStickY} fill="#444" r="10"/>

        <!-- C Stick -->
        <circle cx="130" cy="100" fill="#111" r="12"/>
        <circle cx={130 + cStickX} cy={100 + cStickY} fill="#E1C655" r="8"/>

        <!-- ABXY Buttons (GC style) -->
        <!-- A Button (large, green, center) -->
        <circle cx="155" cy="55" fill={isActive('A') ? '#4CAF50' : '#2E5030'} r="12"/>

        <!-- B Button (small, red, bottom-left) -->
        <circle cx="135" cy="70" fill={isActive('B') ? '#F44336' : '#6A2A27'} r="6"/>

        <!-- Y Button (bean shape, top arc around A) -->
        <path d="M 142 36 A 15 15 0 0 1 168 36 A 4 4 0 0 1 166 42 A 18 18 0 0 0 144 42 A 4 4 0 0 1 142 36 Z"
              fill={isActive('Y') ? '#9E9E9E' : '#444'}/>

        <!-- X Button (bean shape, right arc around A) -->
        <path d="M 174 42 A 15 15 0 0 1 174 68 A 4 4 0 0 1 168 66 A 18 18 0 0 0 168 44 A 4 4 0 0 1 174 42 Z"
              fill={isActive('X') ? '#9E9E9E' : '#444'}/>

        <!-- Start Button -->
        <circle cx="100" cy="55" fill={isActive('Plus') ? 'var(--accent-color)' : '#444'} r="5"/>
    </svg>
</div>

<style>
    .gc-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 8px;
    }

    .view-label {
        font-size: 10px;
        color: var(--text-muted);
        text-align: center;
        margin-bottom: -4px;
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }
</style>
