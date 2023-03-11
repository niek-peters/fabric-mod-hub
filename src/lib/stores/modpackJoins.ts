import { writable } from "svelte/store";

export const modpackJoins = writable<ModpackJoin[]>();

export function setModpackJoins(newModpackJoins: ModpackJoin[]) {
  modpackJoins.set(newModpackJoins);
}