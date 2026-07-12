<script lang="ts">
    import {onDestroy, onMount} from 'svelte';
    import {invoke} from '@tauri-apps/api/core';
    import {listen} from '@tauri-apps/api/event';
    import type {UnlistenFn} from '@tauri-apps/api/event';
    import Tabs from '$lib/components/Tabs.svelte';
    import ProfilesTab from '$lib/components/ProfilesTab.svelte';
    import ConnectionsTab from '$lib/components/ConnectionsTab.svelte';
    import ControllersTab from '$lib/components/ControllersTab.svelte';
    import ProfileEditor from '$lib/components/ProfileEditor.svelte';
    import SettingsTab from '$lib/components/SettingsTab.svelte';
    import {removeConnectionWithVirtualControllerCleanup} from '$lib/connectionRemoval';

    const TABS = ["Controllers", "Profiles", "Connections", "Settings"];
    let activeTab = "Profiles";

    // State for Profiles tab
    let editingProfileName: string | null = null;
    let isEditingProfile = false;
    let unlistenRemoveConnection: UnlistenFn | null = null;

    onMount(async () => {
        unlistenRemoveConnection = await listen<string>('remove-connection', async event => {
            await removeConnectionWithVirtualControllerCleanup(event.payload, async () => {
                try {
                    await invoke('disconnect_controller', {id: event.payload});
                } catch (error) {
                    // The physical connection is already gone; keep local cleanup reliable.
                    console.warn(`Failed to release disconnected controller ${event.payload}`, error);
                }
            });
        });
    });

    onDestroy(() => {
        unlistenRemoveConnection?.();
    });

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
        {:else if activeTab === "Settings"}
            <SettingsTab />
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
