import type { LayoutLoad } from './$types';
import { invoke } from '@tauri-apps/api/tauri';

export const load: LayoutLoad = async () => {
	const res = await invoke('get_all_modpacks');

	// Check if res is an array of modpackjoins
	let modpacks: Modpack[] = [];
	if (Array.isArray(res)) {
		modpacks = res;
	}

	return {
		modpacks
	};
};
