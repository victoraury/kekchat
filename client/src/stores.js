import { writable } from 'svelte/store';

export const isLoggedIn = writable(false);
export const username = writable('');
export const users = writable([]);
export const messages = writable([]);
export const loginAttemptStatus = writable('');