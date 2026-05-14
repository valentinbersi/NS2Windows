<script>
    export let width = "24";
    export let height = "24";
    export let fill = "currentColor";
</script>

<svg {width} {height} viewBox="0 0 24 24" fill="none" stroke={fill} stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <!-- Pro Controller Body -->
    <path d="M21 11C21 15.4183 17.4183 19 13 19H11C6.58172 19 3 15.4183 3 11V8C3 5.23858 5.23858 3 8 3H16C18.7614 3 21 5.23858 21 8V11Z"/>
    <!-- Grips -->
    <path d="M3 11C3 13.5 2 17 2 20C2 21.1046 2.89543 22 4 22C6 22 7.5 19 8.5 17"/>
    <path d="M21 11C21 13.5 22 17 22 20C22 21.1046 21.1046 22 20 22C18 22 16.5 19 15.5 17"/>
    
    <!-- D-pad -->
    <path d="M8 12V14M7 13H9"/>
    
    <!-- Right Buttons -->
    <circle cx="16" cy="7" r="0.8" fill={fill}/>
    <circle cx="18" cy="9" r="0.8" fill={fill}/>
    <circle cx="14" cy="9" r="0.8" fill={fill}/>
    <circle cx="16" cy="11" r="0.8" fill={fill}/>
    
    <!-- Left Stick -->
    <circle cx="8" cy="8" r="1.5" fill={fill}/>
    
    <!-- Right Stick -->
    <circle cx="14" cy="14" r="1.5" fill={fill}/>
</svg>
