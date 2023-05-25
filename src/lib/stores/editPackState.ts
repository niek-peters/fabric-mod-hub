import { writable } from 'svelte/store';

const defaultState: EditModpackVersion = {
	custom_filepaths: []
};

export const editingPack = writable<boolean>(false);
export const editPackState = writable<EditModpackVersion>(structuredClone(defaultState));

export function setEditState(state: EditModpackVersion) {
	editPackState.set(state);
}

export function resetEditState() {
	editPackState.set(structuredClone(defaultState));
}

export function addFilepath(filepath: string) {
	editPackState.update((state) => {
		state.custom_filepaths.push(filepath);
		return state;
	});
}

export function removeFilepath(filepath: string) {
	editPackState.update((state) => {
		state.custom_filepaths = state.custom_filepaths.filter((m) => m !== filepath);
		return state;
	});
}

export function startEditing() {
	editingPack.set(true);
}

export function stopEditing() {
	editingPack.set(false);
}
