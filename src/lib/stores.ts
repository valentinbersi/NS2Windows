import { writable } from 'svelte/store';
import type { Connection, VirtualControllerState } from './types';

// Store for all active connections
export const connections = writable<Connection[]>([]);

// Store for virtual controllers
export const virtualControllers = writable<VirtualControllerState[]>([]);
