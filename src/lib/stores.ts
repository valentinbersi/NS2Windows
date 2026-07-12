import { get, writable } from 'svelte/store';
import { CONTROLLER_KIND_LABELS } from './types';
import type { Connection, VirtualControllerState } from './types';

// Store for all active connections
export const connections = writable<Connection[]>([]);

// Store for virtual controllers
export const virtualControllers = writable<VirtualControllerState[]>([]);

export type NewConnection = Omit<Connection, 'name' | 'is_custom_name'>;

export type RenameConnectionResult =
    | { ok: true; name: string }
    | { ok: false; error: string };

function normalizeName(name: string): string {
    return name.trim().toLowerCase();
}

function nextAvailableDefaultName(baseName: string, occupiedNames: Set<string>): string {
    let suffix = 1;
    let candidate = `${baseName} (${suffix})`;

    while (occupiedNames.has(normalizeName(candidate))) {
        suffix += 1;
        candidate = `${baseName} (${suffix})`;
    }

    return candidate;
}

function reconcileAutomaticNames(currentConnections: Connection[]): Connection[] {
    const kindCounts = new Map<Connection['controller_kind'], number>();
    const occupiedNames = new Set(
        currentConnections
            .filter(connection => connection.is_custom_name)
            .map(connection => normalizeName(connection.name))
    );

    for (const connection of currentConnections) {
        kindCounts.set(connection.controller_kind, (kindCounts.get(connection.controller_kind) ?? 0) + 1);
    }

    return currentConnections.map(connection => {
        if (connection.is_custom_name) {
            return connection;
        }

        const baseName = CONTROLLER_KIND_LABELS[connection.controller_kind];
        const hasSameKindConnections = (kindCounts.get(connection.controller_kind) ?? 0) > 1;
        let name = baseName;

        if (hasSameKindConnections || occupiedNames.has(normalizeName(name))) {
            name = nextAvailableDefaultName(baseName, occupiedNames);
        }

        occupiedNames.add(normalizeName(name));
        return {...connection, name};
    });
}

function synchronizeBoundControllers(currentConnections: Connection[]) {
    const connectionsById = new Map(currentConnections.map(connection => [connection.id, connection]));

    virtualControllers.update(controllers => controllers.map(controller => ({
        ...controller,
        bound_controllers: controller.bound_controllers.map(bound => connectionsById.get(bound.id) ?? bound),
    })));
}

function setConnections(nextConnections: Connection[]) {
    const reconciledConnections = reconcileAutomaticNames(nextConnections);
    connections.set(reconciledConnections);
    synchronizeBoundControllers(reconciledConnections);
}

export function addConnection(connection: NewConnection) {
    setConnections([
        ...get(connections),
        {...connection, name: CONTROLLER_KIND_LABELS[connection.controller_kind], is_custom_name: false},
    ]);
}

export function removeConnection(id: string) {
    setConnections(get(connections).filter(connection => connection.id !== id));
}

export function renameConnection(id: string, requestedName: string): RenameConnectionResult {
    const trimmedName = requestedName.trim();
    const currentConnections = get(connections);
    const connection = currentConnections.find(candidate => candidate.id === id);

    if (!connection) {
        return {ok: false, error: 'This controller is no longer connected.'};
    }

    if (!trimmedName) {
        return {ok: false, error: 'Controller name cannot be empty.'};
    }

    const normalizedName = normalizeName(trimmedName);
    const collision = currentConnections.some(candidate => (
        candidate.id !== id && normalizeName(candidate.name) === normalizedName
    ));

    if (collision) {
        return {ok: false, error: 'Another connected controller already uses this name.'};
    }

    if (trimmedName === connection.name) {
        return {ok: true, name: connection.name};
    }

    setConnections(currentConnections.map(candidate => (
        candidate.id === id
            ? {...candidate, name: trimmedName, is_custom_name: true}
            : candidate
    )));

    return {ok: true, name: trimmedName};
}
