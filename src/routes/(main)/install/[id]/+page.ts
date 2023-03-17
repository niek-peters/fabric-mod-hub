import type { PageLoad } from './$types';
import { invoke } from '@tauri-apps/api/tauri';

export const load: PageLoad = async ({ params, parent }) => {
	const id = parseInt(params.id);
	const modpack = (await parent()).modpacks.filter((modpack) => modpack.id === id)[0];

	const res = await invoke('get_modpack_game_versions', { id });

	// Check if res is an array of strings
	let game_versions: string[] = [];
	if (Array.isArray(res)) {
		game_versions = res;
	}

	return {
		id,
		modpack,
		game_versions
	};
};
