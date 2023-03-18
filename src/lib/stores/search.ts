import { writable } from 'svelte/store';

export const search = writable('');

export function setSearch(value: string) {
	search.set(value);
}

export function clearSearch() {
	search.set('');
}
