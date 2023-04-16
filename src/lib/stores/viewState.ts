import { writable } from 'svelte/store';

const defaultState: ViewModpack = {
	modpack: {
		id: 0,
		name: '',
		slug: '',
		premade: false
	},
	mods: []
};

export const viewing = writable<boolean>(false);
export const viewState = writable<ViewModpack>(structuredClone(defaultState));

export function setViewState(state: ViewModpack) {
	viewState.set(state);
}

export function resetViewState() {
	viewState.set(structuredClone(defaultState));
}

export function startViewing() {
	viewing.set(true);
}

export function stopViewing() {
	viewing.set(false);
}
