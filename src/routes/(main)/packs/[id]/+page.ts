import type { PageLoad } from './$types';

import { invoke } from '@tauri-apps/api/tauri';

export const load: PageLoad = async ({ params }) => {
	const id = parseInt(params.id);

	const res = await invoke('get_mod_joins', { id });

	// Check if res is an array of modpackjoins
	let mods: ModJoin[] = [];
	if (Array.isArray(res)) {
		mods = res;
	}

	return {
		id,
		mods
	};
};
