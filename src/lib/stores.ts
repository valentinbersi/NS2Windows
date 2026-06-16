import { writable } from 'svelte/store';
import type { Connection } from './types';

// Store for all active connections
export const connections = writable<Connection[]>([]);
