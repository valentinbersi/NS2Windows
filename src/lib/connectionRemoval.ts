import { invoke } from '@tauri-apps/api/core';
import { get } from 'svelte/store';
import { removeConnection, virtualControllers } from './stores';

const pendingRemovals = new Map<string, Promise<void>>();

/**
 * Stops every virtual controller using a physical connection, unbinds that
 * connection, and removes it from the active connection list.
 *
 * The backend may emit a removal after the device has already gone away, so a
 * failed stop request must not prevent the local state from being cleaned up.
 */
export function removeConnectionWithVirtualControllerCleanup(
    id: string,
    disconnect?: () => Promise<void>,
): Promise<void> {
    const pendingRemoval = pendingRemovals.get(id);
    if (pendingRemoval) {
        return pendingRemoval;
    }

    const removal = (async () => {
        const affectedControllers = get(virtualControllers).filter(controller =>
            controller.bound_controllers.some(boundController => boundController.id === id)
        );

        for (const controller of affectedControllers) {
            if (!controller.is_running || !controller.emulated_controller_id) {
                continue;
            }

            try {
                await invoke('stop_controller', { emulatedControllerId: controller.emulated_controller_id });
            } catch (error) {
                console.error(`Failed to stop virtual controller ${controller.id} while removing connection ${id}`, error);
            }
        }

        if (disconnect) {
            await disconnect();
        }

        virtualControllers.update(controllers => controllers.map(controller => {
            const isAffected = controller.bound_controllers.some(boundController => boundController.id === id);

            return isAffected
                ? {
                    ...controller,
                    bound_controllers: controller.bound_controllers.filter(boundController => boundController.id !== id),
                    is_running: false,
                    emulated_controller_id: null,
                }
                : controller;
        }));
        removeConnection(id);
    })();

    pendingRemovals.set(id, removal);
    void removal.then(
        () => pendingRemovals.delete(id),
        () => pendingRemovals.delete(id),
    );
    return removal;
}
