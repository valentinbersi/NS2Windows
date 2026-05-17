<script>
    export let width = "24";
    export let height = "24";
    export let fill = "currentColor";
</script>

<svg {width} {height} viewBox="0 0 24 24" fill="none" stroke={fill} stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <!-- Body -->
    <path d="M19 8C19 8 21 16 21 18C21 20 19 22 17 21C15 20 13 16 12 16C11 16 9 20 7 21C5 22 3 20 3 18C3 16 5 8 5 8C5 5 8 4 12 4C16 4 19 5 19 8Z"/>
    <!-- Guide Button -->
    <circle cx="12" cy="8" r="1.5" fill={fill}/>
    <!-- D-pad (Lower left) -->
    <path d="M8 12V14M7 13H9"/>
    <!-- Action Buttons (Upper right) -->
    <circle cx="17" cy="8" r="1.5" fill={fill}/>
    <!-- Left Stick (Upper left) -->
    <circle cx="7" cy="8" r="1.5" fill={fill}/>
    <!-- Right Stick (Lower right) -->
    <circle cx="16" cy="13" r="1.5" fill={fill}/>
</svg>
