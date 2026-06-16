<script lang="ts">
    import type { NsInput } from "../types";
    export let inputs: Partial<Record<NsInput, number>> = {};
    $: isActive = (input: NsInput) => (inputs[input] || 0) > 0.5;
    
    $: lStickX = ((inputs.LeftXPlus || 0) + (inputs.LeftXMinus || 0)) * 8;
    $: lStickY = ((inputs.LeftYMinus || 0) + (inputs.LeftYPlus || 0)) * 8;
    $: rStickX = ((inputs.RightXPlus || 0) + (inputs.RightXMinus || 0)) * 8;
    $: rStickY = ((inputs.RightYMinus || 0) + (inputs.RightYPlus || 0)) * 8;
</script>
<svg viewBox="0 0 200 150" width="200" height="150" style="max-height: 200px;">
    <!-- Body roughly like Pro Controller -->
    <path d="M 40 20 Q 100 20 160 20 Q 180 20 190 50 Q 200 90 170 130 Q 150 150 140 120 Q 130 90 100 90 Q 70 90 60 120 Q 50 150 30 130 Q 0 90 10 50 Q 20 20 40 20 Z" fill="var(--bg-surface-hover)" stroke="var(--border-color)" stroke-width="2"/>
    
    <!-- D-pad -->
    <rect x="55" y="85" width="20" height="8" rx="2" fill={isActive('Right') || isActive('Left') ? 'var(--accent-color)' : '#444'} transform="rotate(90 65 89)"/>
    <rect x="55" y="85" width="20" height="8" rx="2" fill={isActive('Up') || isActive('Down') ? 'var(--accent-color)' : '#444'} />
    <circle cx="65" cy="80" r="4" fill={isActive('Up') ? 'var(--accent-color)' : 'transparent'} />
    <circle cx="65" cy="98" r="4" fill={isActive('Down') ? 'var(--accent-color)' : 'transparent'} />
    <circle cx="56" cy="89" r="4" fill={isActive('Left') ? 'var(--accent-color)' : 'transparent'} />
    <circle cx="74" cy="89" r="4" fill={isActive('Right') ? 'var(--accent-color)' : 'transparent'} />
    
    <!-- ABXY Buttons -->
    <circle cx="150" cy="55" r="20" fill="none" stroke="rgba(255,255,255,0.05)" />
    <circle cx="150" cy="40" r="5" fill={isActive('X') ? 'var(--accent-color)' : '#444'} />
    <circle cx="150" cy="70" r="5" fill={isActive('B') ? 'var(--accent-color)' : '#444'} />
    <circle cx="135" cy="55" r="5" fill={isActive('Y') ? 'var(--accent-color)' : '#444'} />
    <circle cx="165" cy="55" r="5" fill={isActive('A') ? 'var(--accent-color)' : '#444'} />
    
    <!-- L Stick -->
    <circle cx="55" cy="55" r="14" fill="#111"/>
    <circle cx={55 + lStickX} cy={55 + lStickY} r="10" fill={isActive('LeftXPlus') || isActive('LeftXMinus') || isActive('LeftYPlus') || isActive('LeftYMinus') ? 'var(--accent-color)' : '#444'}/>
    
    <!-- R Stick -->
    <circle cx="130" cy="100" r="14" fill="#111"/>
    <circle cx={130 + rStickX} cy={100 + rStickY} r="10" fill={isActive('RightXPlus') || isActive('RightXMinus') || isActive('RightYPlus') || isActive('RightYMinus') ? 'var(--accent-color)' : '#444'}/>
    
    <!-- Minus & Plus -->
    <rect x="75" y="35" width="10" height="3" rx="1" fill={isActive('Minus') ? 'var(--accent-color)' : '#444'} />
    <path d="M 115 36 L 125 36 M 120 31 L 120 41" stroke={isActive('Plus') ? 'var(--accent-color)' : '#444'} stroke-width="3" stroke-linecap="round" />
    
    <!-- Home & Capture -->
    <rect x="85" y="60" width="8" height="8" rx="1" fill={isActive('Capture') ? 'var(--accent-color)' : '#444'} />
    <circle cx="110" cy="64" r="5" fill={isActive('Home') ? 'var(--accent-color)' : '#444'} />
    
    <!-- Shoulders -->
    <path d="M 30 30 Q 40 10 70 10 L 70 15 Q 45 15 35 30 Z" fill={isActive('L') ? 'var(--accent-color)' : '#444'} />
    <path d="M 25 25 Q 40 0 75 0 L 75 5 Q 45 5 30 25 Z" fill={isActive('Zl') ? 'var(--accent-color)' : '#444'} />
    
    <path d="M 170 30 Q 160 10 130 10 L 130 15 Q 155 15 165 30 Z" fill={isActive('R') ? 'var(--accent-color)' : '#444'} />
    <path d="M 175 25 Q 160 0 125 0 L 125 5 Q 155 5 170 25 Z" fill={isActive('Zr') ? 'var(--accent-color)' : '#444'} />
</svg>
