<script lang="ts">
    import type {NsInput} from "../types";

    export let inputs: Partial<Record<NsInput, number>> = {};
    $: isActive = (input: NsInput) => (inputs[input] || 0) > 0.5;

    $: stickX = ((inputs.RightXPlus || 0) + -(inputs.RightXMinus || 0)) * 10;
    $: stickY = -((-(inputs.RightYMinus || 0) + (inputs.RightYPlus || 0)) * 10);
</script>

<div class="joycon-container">
    <!-- Top View -->
    <div class="view-label">Top</div>
    <svg class="top-view" height="30" viewBox="0 0 60 30" width="60">
        <!-- ZR (Back) -->
        <rect fill={isActive('Zr') ? 'var(--accent-color)' : '#444'} height="12" rx="3" width="35" x="10" y="2"/>
        <!-- Body -->
        <path d="M 0 10 L 45 10 Q 60 10 60 20 L 60 30 L 0 30 Z" fill="var(--bg-surface-hover)"
              stroke="var(--border-color)" stroke-width="2"/>
        <!-- R (Front) -->
        <path d="M 0 10 L 45 10 Q 60 10 60 20 L 55 20 Q 55 16 45 16 L 0 16 Z"
              fill={isActive('R') ? 'var(--accent-color)' : '#444'}/>
    </svg>

    <div class="main-views">
        <!-- Side View (Inner Rail) -->
        <div class="side-view-container">
            <div class="view-label">Side</div>
            <svg class="side-view" height="180" viewBox="0 0 20 180" width="20">
                <!-- Rail Body -->
                <rect fill="#222" height="150" rx="2" width="10" x="5" y="15"/>
                <!-- SL -->
                <rect fill={isActive('Sr') ? 'var(--accent-color)' : '#444'} height="25" rx="3" width="8" x="6" y="40"/>
                <!-- SR -->
                <rect fill={isActive('Sl') ? 'var(--accent-color)' : '#444'} height="25" rx="3" width="8" x="6"
                      y="115"/>
            </svg>
        </div>

        <!-- Front View -->
        <div class="front-view-container">
            <div class="view-label">Front</div>
            <svg height="180" viewBox="0 0 60 180" width="60">
                <!-- Body (Flat Left, Rounded Right) -->
                <path d="M 5 5 L 30 5 Q 55 5 55 30 L 55 150 Q 55 175 30 175 L 5 175 Z" fill="var(--bg-surface-hover)"
                      stroke="var(--border-color)" stroke-width="2"/>

                <!-- Plus -->
                <path d="M 15 16 L 25 16 M 20 11 L 20 21" stroke={isActive('Plus') ? 'var(--accent-color)' : '#444'}
                      stroke-linecap="round" stroke-width="3"/>

                <!-- Buttons ABXY -->
                <circle cx="30" cy="50" fill="none" r="22" stroke="rgba(255,255,255,0.05)"/>
                <circle cx="30" cy="34" fill={isActive('X') ? 'var(--accent-color)' : '#444'} r="5"/>
                <circle cx="30" cy="66" fill={isActive('B') ? 'var(--accent-color)' : '#444'} r="5"/>
                <circle cx="14" cy="50" fill={isActive('Y') ? 'var(--accent-color)' : '#444'} r="5"/>
                <circle cx="46" cy="50" fill={isActive('A') ? 'var(--accent-color)' : '#444'} r="5"/>

                <!-- Stick Background -->
                <circle cx="30" cy="110" fill="#111" r="14"/>
                <!-- Stick Knob -->
                <circle cx={30 + stickX} cy={110 + stickY} fill={isActive('Tr') ? 'var(--accent-color)' : '#444'}
                        r="10"/>

                <!-- Home -->
                <circle cx="20" cy="150" fill={isActive('Home') ? 'var(--accent-color)' : '#444'} r="5"/>
                <!-- Chat (square, below home) -->
                <rect fill={isActive('Chat') ? 'var(--accent-color)' : '#444'} height="8" rx="1" width="8" x="16"
                      y="162"/>
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
