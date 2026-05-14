<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    
    export let onEditProfile: (name: string | null) => void;

    let profiles: string[] = [];
    let searchQuery = "";
    let loading = true;

    async function loadProfiles() {
        loading = true;
        try {
            profiles = await invoke<string[]>("profile_names");
        } catch (e) {
            console.error("Failed to load profiles", e);
        } finally {
            loading = false;
        }
    }

    onMount(() => {
        loadProfiles();
    });

    $: filteredProfiles = profiles.filter(p => 
        p.toLowerCase().includes(searchQuery.toLowerCase())
    );

    async function deleteProfile(name: string, event: Event) {
        event.stopPropagation(); // Prevent triggering the edit action
        if (confirm(`Are you sure you want to delete the profile "${name}"?`)) {
            try {
                await invoke("delete_profile", { name });
                await loadProfiles();
            } catch (e) {
                console.error("Failed to delete profile", e);
                alert("Failed to delete profile");
            }
        }
    }
</script>

<div class="tab-container">
    <div class="toolbar">
        <div class="search-box">
            <span class="search-icon">⚲</span>
            <input 
                type="text" 
                placeholder="Search profiles..." 
                bind:value={searchQuery}
            />
        </div>
        <button class="primary" on:click={() => onEditProfile(null)}>
            + New Profile
        </button>
    </div>

    {#if loading}
        <div class="empty-state">Loading profiles...</div>
    {:else if profiles.length === 0}
        <div class="empty-state">
            <p>No profiles found.</p>
            <button class="primary" on:click={() => onEditProfile(null)}>Create your first profile</button>
        </div>
    {:else if filteredProfiles.length === 0}
        <div class="empty-state">
            <p>No profiles match your search.</p>
        </div>
    {:else}
        <div class="profiles-list">
            {#each filteredProfiles as profile}
                <button class="profile-card" on:click={() => onEditProfile(profile)}>
                    <div class="profile-info">
                        <span class="profile-name">{profile}</span>
                    </div>
                    <div class="profile-actions">
                        <span class="action-btn edit-btn" title="Edit Profile">✎</span>
                        <span 
                            class="action-btn delete-btn" 
                            title="Delete Profile"
                            role="button"
                            tabindex="0"
                            on:click={(e) => deleteProfile(profile, e)}
                            on:keydown={(e) => e.key === 'Enter' && deleteProfile(profile, e)}
                        >🗑</span>
                    </div>
                </button>
            {/each}
        </div>
    {/if}
</div>

<style>
    .tab-container {
        display: flex;
        flex-direction: column;
        height: 100%;
        padding: 24px 32px;
        gap: 24px;
        overflow: hidden;
    }

    .toolbar {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 16px;
    }

    .search-box {
        position: relative;
        flex: 1;
        max-width: 400px;
    }

    .search-icon {
        position: absolute;
        left: 12px;
        top: 50%;
        transform: translateY(-50%) rotate(-45deg);
        color: var(--text-muted);
        font-size: 16px;
        pointer-events: none;
    }

    .search-box input {
        width: 100%;
        padding-left: 36px;
        border-radius: 20px;
    }

    .empty-state {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        color: var(--text-muted);
        gap: 16px;
    }

    .profiles-list {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
        gap: 16px;
        overflow-y: auto;
        padding-bottom: 24px;
    }

    .profile-card {
        display: flex;
        justify-content: space-between;
        align-items: center;
        background: var(--bg-surface);
        border: 1px solid var(--border-color);
        border-radius: 8px;
        padding: 16px 20px;
        text-align: left;
        transition: all 0.2s ease;
        position: relative;
    }

    .profile-card:hover {
        border-color: var(--text-muted);
        transform: translateY(-2px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    }

    .profile-info {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }

    .profile-name {
        font-size: 16px;
        font-weight: 500;
        color: var(--text-color);
    }

    .profile-actions {
        display: flex;
        gap: 8px;
        opacity: 0;
        transition: opacity 0.2s ease;
    }

    .profile-card:hover .profile-actions {
        opacity: 1;
    }

    .action-btn {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 32px;
        height: 32px;
        border-radius: 50%;
        background: var(--bg-color);
        color: var(--text-muted);
        font-size: 14px;
        transition: all 0.2s;
    }

    .action-btn:hover {
        color: var(--text-color);
        background: var(--border-color);
    }

    .delete-btn:hover {
        color: var(--danger-color);
        background: rgba(255, 68, 68, 0.1);
    }
</style>
