<script>
    export let width = "24";
    export let height = "24";
    export let fill = "currentColor";
</script>

<svg {width} {height} viewBox="0 0 24 24" fill="none" stroke={fill} stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <!-- Left Joy-Con -->
    <path d="M10 2H6C3.23858 2 1 4.23858 1 7V17C1 19.7614 3.23858 22 6 22H10V2Z"/>
    <circle cx="5.5" cy="7" r="1.5" fill={fill}/>
    <circle cx="5.5" cy="12" r="0.6" fill={fill}/>
    <circle cx="5.5" cy="16" r="0.6" fill={fill}/>
    <circle cx="3.5" cy="14" r="0.6" fill={fill}/>
    <circle cx="7.5" cy="14" r="0.6" fill={fill}/>
    
    <!-- Right Joy-Con -->
    <path d="M14 2H18C20.7614 2 23 4.23858 23 7V17C23 19.7614 20.7614 22 18 22H14V2Z"/>
    <circle cx="18.5" cy="7" r="0.6" fill={fill}/>
    <circle cx="18.5" cy="11" r="0.6" fill={fill}/>
    <circle cx="16.5" cy="9" r="0.6" fill={fill}/>
    <circle cx="20.5" cy="9" r="0.6" fill={fill}/>
    <circle cx="18.5" cy="15" r="1.5" fill={fill}/>
</svg>
