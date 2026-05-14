<script>
    export let width = "24";
    export let height = "24";
    export let fill = "currentColor";
</script>

<svg {width} {height} viewBox="0 0 24 24" fill="none" stroke={fill} stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <!-- Left Joy-Con Body (flat on right side, rounded on left) -->
    <path d="M11 2H7C4.23858 2 2 4.23858 2 7V17C2 19.7614 4.23858 22 7 22H11V2Z"/>
    <!-- Stick -->
    <circle cx="6.5" cy="7" r="1.5" fill={fill}/>
    <!-- D-pad buttons -->
    <circle cx="6.5" cy="12" r="0.8" fill={fill}/>
    <circle cx="6.5" cy="16" r="0.8" fill={fill}/>
    <circle cx="4.5" cy="14" r="0.8" fill={fill}/>
    <circle cx="8.5" cy="14" r="0.8" fill={fill}/>
    <!-- Minus button -->
    <line x1="5" y1="3.5" x2="8" y2="3.5" stroke-width="1.5"/>
</svg>
