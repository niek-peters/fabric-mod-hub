import { invoke } from '@tauri-apps/api/tauri';

const res = await invoke('get_all_modpack_versions');

// Check if res is an array of modpackjoins
let modpacks: ModpackJoin[] = [];
if (Array.isArray(res)) {
	modpacks = res;
}

export { modpacks };
