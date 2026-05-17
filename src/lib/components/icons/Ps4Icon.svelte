<script>
    export let width = "24";
    export let height = "24";
    export let fill = "currentColor";
</script>

<svg {width} {height} viewBox="0 0 24 24" fill="none" stroke={fill} stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <!-- Body -->
    <path d="M19 8C19 8 20 15 21 17C22 19 21 21 19 21C17 21 15 17 15 17L14 14H10L9 17C9 17 7 21 5 21C3 21 2 19 3 17C4 15 5 8 5 8C5 5 8 4 12 4C16 4 19 5 19 8Z"/>
    <!-- Touchpad -->
    <rect x="9" y="5" width="6" height="4" rx="1"/>
    <!-- D-pad -->
    <path d="M6 8V10M5 9H7"/>
    <!-- Action Buttons -->
    <circle cx="18" cy="9" r="1.5" fill={fill}/>
    <!-- Sticks -->
    <circle cx="9" cy="14" r="1.5" fill={fill}/>
    <circle cx="15" cy="14" r="1.5" fill={fill}/>
</svg>
