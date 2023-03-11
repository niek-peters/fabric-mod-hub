import { get, writable } from 'svelte/store';

export const modpackJoins = writable<ModpackJoin[]>();

export function setModpackJoins(newModpackJoins: ModpackJoin[]) {
	modpackJoins.set(newModpackJoins);
}

export function getByVersionId(versionId: number): ModpackJoin | undefined {
	const joins = get(modpackJoins);
	return joins.find((join) => join.id === versionId);
}
