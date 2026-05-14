<script lang="ts">
    import Tabs from '$lib/components/Tabs.svelte';
    import ProfilesTab from '$lib/components/ProfilesTab.svelte';
    import ConnectionsTab from '$lib/components/ConnectionsTab.svelte';
    import ControllersTab from '$lib/components/ControllersTab.svelte';
    import ProfileEditor from '$lib/components/ProfileEditor.svelte';

    const TABS = ["Controllers", "Profiles", "Connections"];
    let activeTab = "Profiles";

    // State for Profiles tab
    let editingProfileName: string | null = null;
    let isEditingProfile = false;

    function handleTabChange(tab: string) {
        activeTab = tab;
        // Reset sub-state when changing tabs
        isEditingProfile = false;
        editingProfileName = null;
    }

    function handleEditProfile(name: string | null) {
        editingProfileName = name;
        isEditingProfile = true;
    }

    function handleBackFromEditor() {
        isEditingProfile = false;
        editingProfileName = null;
        // Trigger a reload of the profiles list if necessary by forcing a remount or using stores,
        // but for now, switching back will re-mount ProfilesTab, which calls loadProfiles.
    }
</script>

<main id="app">
    <Tabs tabs={TABS} {activeTab} onTabChange={handleTabChange} />
    
    <div class="content-area">
        {#if activeTab === "Controllers"}
            <ControllersTab />
        {:else if activeTab === "Profiles"}
            {#if isEditingProfile}
                <ProfileEditor 
                    profileName={editingProfileName} 
                    onBack={handleBackFromEditor} 
                />
            {:else}
                <ProfilesTab onEditProfile={handleEditProfile} />
            {/if}
        {:else if activeTab === "Connections"}
            <ConnectionsTab />
        {/if}
    </div>
</main>

<style>
    .content-area {
        flex: 1;
        overflow: hidden;
        position: relative;
    }
</style>
