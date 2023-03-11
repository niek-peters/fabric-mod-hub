import type { PageLoad } from './$types';

import { invoke } from '@tauri-apps/api/tauri';
import { getByVersionId } from '$stores/modpackJoins';

export const load: PageLoad = async ({ params }) => {
	const id = parseInt(params.id);

	const res = await invoke('get_mod_joins_from_modpack_version_id', { id });

	// Check if res is an array of modpackjoins
	let mods: ModJoin[] = [];
	if (Array.isArray(res)) {
		mods = res;
	}

	return {
		id,
		modpack: getByVersionId(id),
		mods
	};
};
