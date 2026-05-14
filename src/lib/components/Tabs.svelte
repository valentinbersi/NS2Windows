<script lang="ts">
    export let tabs: string[] = [];
    export let activeTab: string;
    export let onTabChange: (tab: string) => void;
</script>

<div class="tabs-container">
    {#each tabs as tab}
        <button 
            class="tab-btn {activeTab === tab ? 'active' : ''}" 
            on:click={() => onTabChange(tab)}
        >
            {tab}
        </button>
    {/each}
</div>

<style>
    .tabs-container {
        display: flex;
        gap: 8px;
        padding: 16px 24px;
        background-color: var(--bg-surface);
        border-bottom: 1px solid var(--border-color);
        box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
    }

    .tab-btn {
        background: transparent;
        border: none;
        border-radius: 6px;
        padding: 8px 16px;
        color: var(--text-muted);
        font-weight: 500;
        transition: all 0.2s;
    }

    .tab-btn:hover {
        background: var(--bg-surface-hover);
        color: var(--text-color);
    }

    .tab-btn.active {
        background: var(--bg-color);
        color: var(--accent-color);
        box-shadow: 0 0 0 1px var(--border-color);
    }
</style>
