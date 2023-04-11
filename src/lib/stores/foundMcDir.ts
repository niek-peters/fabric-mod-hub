import { writable } from 'svelte/store';

export const foundMcDir = writable(true);

export function setFoundMcDir(found: boolean) {
	foundMcDir.set(found);
}
