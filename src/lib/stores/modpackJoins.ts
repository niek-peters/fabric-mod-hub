import { invoke } from '@tauri-apps/api/tauri';
import { get, writable } from 'svelte/store';

export const modpackJoins = writable<ModpackJoin[]>([]);

export function setModpackJoins(newModpackJoins: ModpackJoin[]) {
	modpackJoins.set(newModpackJoins);
}

export function fromVersionId(versionId: number): ModpackJoin {
	const joins = get(modpackJoins);
	const modpackJoin = joins.find((join) => join.id === versionId);

	if (!modpackJoin)
		throw new Error('Modpack join with specified version id does not exist in store');

	return modpackJoin;
}

export async function loadFromVersionId(versionId: number) {
	console.log('Loading...');

	await invoke('load_modpack_version', { id: versionId });

	modpackJoins.update((joins) => {
		for (const join of joins) {
			join.loaded = false;
		}

		const modpackJoin = joins.find((join) => join.id === versionId);
		if (!modpackJoin) return joins;
		modpackJoin.loaded = true;
		return joins;
	});

	console.log(get(modpackJoins));
}

export async function unload() {
	await invoke('unload_modpack_versions');

	modpackJoins.update((joins) => {
		for (const join of joins) {
			join.loaded = false;
		}
		return joins;
	});
}
