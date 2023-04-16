import { writable } from 'svelte/store';

const defaultState: AddModpack = {
	modpack: {
		id: null,
		name: '',
		slug: '',
		premade: false
	},
	mods: []
};

export const adding = writable<boolean>(false);
export const addState = writable<AddModpack>(structuredClone(defaultState));

export function setAddState(state: AddModpack) {
	addState.set(state);
}

export function resetAddState() {
	addState.set(structuredClone(defaultState));
}

export function setTitle(title: string) {
	addState.update((state) => {
		state.modpack.name = title;
		state.modpack.slug = title.toLowerCase().replace(/ /g, '-');
		return state;
	});
}

export function addMod(mod: NewMod) {
	addState.update((state) => {
		state.mods.push(mod);
		return state;
	});
}

export function removeMod(mod: NewMod) {
	addState.update((state) => {
		state.mods = state.mods.filter((m) => m.project_id !== mod.project_id);
		return state;
	});
}

export function startAdding() {
	adding.set(true);
}

export function stopAdding() {
	adding.set(false);
}
