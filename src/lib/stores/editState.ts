import { writable } from 'svelte/store';

const defaultState: EditModpack = {
	modpack: {
		id: 0,
		name: '',
		slug: '',
		premade: false
	},
	mods: []
};

export const editing = writable<boolean>(false);
export const editState = writable<EditModpack>(structuredClone(defaultState));

export function setEditState(state: EditModpack) {
	editState.set(state);
}

export function resetEditState() {
	editState.set(structuredClone(defaultState));
}

export function setTitle(title: string) {
	editState.update((state) => {
		state.modpack.name = title;
		state.modpack.slug = title.toLowerCase().replace(/ /g, '-');
		return state;
	});
}

export function addMod(mod: NewMod) {
	editState.update((state) => {
		state.mods.push(mod);
		return state;
	});
}

export function removeMod(mod: MixedMod) {
	editState.update((state) => {
		state.mods = state.mods.filter((m) => m.project_id !== mod.project_id);
		return state;
	});
}

export function startEditing() {
	editing.set(true);
}

export function stopEditing() {
	editing.set(false);
}
