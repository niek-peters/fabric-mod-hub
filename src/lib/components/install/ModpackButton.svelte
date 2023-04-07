<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	import { page } from '$app/stores';

	import Fa from 'svelte-fa';
	import { faCaretRight, faUpRightFromSquare, faTrash } from '@fortawesome/free-solid-svg-icons';

	import { modpackJoins, remove, unload } from '$stores/modpackJoins';

	export let id: number;
	export let name: string;
	export let premade = true;
	export let deleteFunction: (id: number) => void = () => {};

	async function deleteModpack() {
		// Get all related modpack joins
		const joins = $modpackJoins.filter((join) => join.modpack_id === id);

		// Uninstall all modpack joins
		for (const join of joins) {
			if (join.loaded) unload();
			await invoke('uninstall_modpack_version', { id: join.id });
			await remove(join.id);
		}

		await invoke('delete_modpack', { id });
		deleteFunction(id);
	}
</script>

<div class="flex gap-4">
	<a
		class="flex justify-between items-center flex-grow transition duration-300 px-4 py-2 rounded-md shadow-2xl text-xl {parseInt(
			$page.params.id
		) === id
			? 'bg-zinc-700/80'
			: 'bg-zinc-700/20 hover:bg-zinc-700/50'}"
		href="/install/{id}"
		>{name}
		<Fa icon={faCaretRight} class="text-3xl" />
	</a>
	{#if !premade}
		<button
			on:click={deleteModpack}
			class="flex flex-col items-center justify-center gap-1 h-14 w-14 bg-rose-800 hover:bg-rose-900 transition duration-300 rounded-md shadow-2xl"
			><Fa class="text-lg" icon={faTrash} />
			<p class="-mb-1 text-sm">Delete</p></button
		>{/if}
	<a
		href="/packs/{id}"
		class="flex flex-col items-center justify-center gap-1 h-14 w-14 bg-indigo-800 hover:bg-indigo-900 transition duration-300 rounded-md shadow-2xl"
		><Fa class="text-lg" icon={faUpRightFromSquare} />
		<p class="-mb-1 text-sm">Info</p></a
	>
</div>
