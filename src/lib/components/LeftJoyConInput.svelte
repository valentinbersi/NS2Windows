<script lang="ts">
    import type {NsInput} from "../types";

    export let inputs: Partial<Record<NsInput, number>> = {};
    $: isActive = (input: NsInput) => (inputs[input] || 0) > 0.5;

    $: stickX = ((inputs.LeftXPlus || 0) + -(inputs.LeftXMinus || 0)) * 10;
    $: stickY = -((-(inputs.LeftYMinus || 0) + (inputs.LeftYPlus || 0)) * 10);
</script>

<div class="joycon-container">
    <!-- Top View -->
    <div class="view-label">Top</div>
    <svg class="top-view" height="30" viewBox="0 0 60 30" width="60">
        <!-- ZL (Back) -->
        <rect fill={isActive('Zl') ? 'var(--accent-color)' : '#444'} height="12" rx="3" width="35" x="15" y="2"/>
        <!-- Body -->
        <path d="M 0 30 L 0 20 Q 0 10 15 10 L 60 10 L 60 30 Z" fill="var(--bg-surface-hover)"
              stroke="var(--border-color)" stroke-width="2"/>
        <!-- L (Front) -->
        <path d="M 0 20 Q 0 10 15 10 L 60 10 L 60 16 L 15 16 Q 5 16 5 20 Z"
              fill={isActive('L') ? 'var(--accent-color)' : '#444'}/>
    </svg>

    <div class="main-views">
        <!-- Front View -->
        <div class="front-view-container">
            <div class="view-label">Front</div>
            <svg height="180" viewBox="0 0 60 180" width="60">
                <!-- Body (Rounded Left, Flat Right) -->
                <path d="M 30 5 Q 5 5 5 30 L 5 150 Q 5 175 30 175 L 55 175 L 55 5 Z" fill="var(--bg-surface-hover)"
                      stroke="var(--border-color)" stroke-width="2"/>

                <!-- Minus -->
                <rect fill={isActive('Minus') ? 'var(--accent-color)' : '#444'} height="3" rx="1.5" width="10" x="35"
                      y="15"/>

                <!-- Stick Background -->
                <circle cx="30" cy="50" fill="#111" r="14"/>
                <!-- Stick Knob -->
                <circle cx={30 + stickX} cy={50 + stickY} fill={isActive('Tl') ? 'var(--accent-color)' : '#444'}
                        r="10"/>

                <!-- Buttons D-pad -->
                <circle cx="30" cy="110" fill="none" r="22" stroke="rgba(255,255,255,0.05)"/>
                <!-- Up -->
                <circle cx="30" cy="94" fill={isActive('Up') ? 'var(--accent-color)' : '#444'} r="5"/>
                <!-- Down -->
                <circle cx="30" cy="126" fill={isActive('Down') ? 'var(--accent-color)' : '#444'} r="5"/>
                <!-- Left -->
                <circle cx="14" cy="110" fill={isActive('Left') ? 'var(--accent-color)' : '#444'} r="5"/>
                <!-- Right -->
                <circle cx="46" cy="110" fill={isActive('Right') ? 'var(--accent-color)' : '#444'} r="5"/>

                <!-- Capture -->
                <rect fill={isActive('Capture') ? 'var(--accent-color)' : '#444'} height="8" rx="2" width="8" x="35"
                      y="150"/>
            </svg>
        </div>

        <!-- Side View (Inner Rail) -->
        <div class="side-view-container">
            <div class="view-label">Side</div>
            <svg class="side-view" height="180" viewBox="0 0 20 180" width="20">
                <!-- Rail Body -->
                <rect fill="#222" height="150" rx="2" width="10" x="5" y="15"/>
                <!-- SL -->
                <rect fill={isActive('Sl') ? 'var(--accent-color)' : '#444'} height="25" rx="3" width="8" x="6" y="40"/>
                <!-- SR -->
                <rect fill={isActive('Sr') ? 'var(--accent-color)' : '#444'} height="25" rx="3" width="8" x="6"
                      y="115"/>
            </svg>
        </div>
    </div>
</div>

<style>
    .joycon-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 8px;
    }

    .main-views {
        display: flex;
        flex-direction: row;
        gap: 16px;
    }

    .view-label {
        font-size: 10px;
        color: var(--text-muted);
        text-align: center;
        margin-bottom: 4px;
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }

    .front-view-container, .side-view-container {
        display: flex;
        flex-direction: column;
    }
</style>
