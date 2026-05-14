<script>
    export let width = "24";
    export let height = "24";
    export let fill = "currentColor";
</script>

<svg {width} {height} viewBox="0 0 24 24" fill="none" stroke={fill} stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <!-- Right Joy-Con Body (flat on left side, rounded on right) -->
    <path d="M13 2H17C19.7614 2 22 4.23858 22 7V17C22 19.7614 19.7614 22 17 22H13V2Z"/>
    <!-- Buttons -->
    <circle cx="17.5" cy="7" r="0.8" fill={fill}/>
    <circle cx="17.5" cy="11" r="0.8" fill={fill}/>
    <circle cx="15.5" cy="9" r="0.8" fill={fill}/>
    <circle cx="19.5" cy="9" r="0.8" fill={fill}/>
    <!-- Stick -->
    <circle cx="17.5" cy="15" r="1.5" fill={fill}/>
    <!-- Plus button -->
    <line x1="16" y1="3.5" x2="19" y2="3.5" stroke-width="1.5"/>
    <line x1="17.5" y1="2" x2="17.5" y2="5" stroke-width="1.5"/>
</svg>
