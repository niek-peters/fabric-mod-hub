import type { PageLoad } from './$types';

import { invoke } from '@tauri-apps/api/tauri';

export const load: PageLoad = async ({ params }) => {
	const id = parseInt(params.id);

	const res1 = await invoke('get_mod_joins', { id });

	// Check if res is an array of modpackjoins
	let mods: ModJoin[] = [];
	if (Array.isArray(res1)) {
		mods = res1;
	}

	const res2 = await invoke('get_modpack_version_custom_filepaths', { id });

	let custom_filepaths: string[] = [];
	if (Array.isArray(res2)) {
		custom_filepaths = res2;
	}

	return {
		id,
		mods,
		custom_filepaths
	};
};
