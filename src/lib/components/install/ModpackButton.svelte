<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	import { page } from '$app/stores';

	import Fa from 'svelte-fa';
	import { faCaretRight, faEye, faPen } from '@fortawesome/free-solid-svg-icons';

	import { setEditState, startEditing } from '$src/lib/stores/editState';
	import { setViewState, startViewing } from '$src/lib/stores/viewState';

	export let id: number;
	export let name: string;
	export let premade = true;

	async function viewModpack() {
		const mods = (await invoke('get_modpack_mods', { id })) as Mod[];

		setViewState({
			modpack: {
				id,
				name,
				premade,
				slug: name.toLowerCase().replace(/ /g, '-')
			},
			mods
		});
		startViewing();
	}

	async function editModpack() {
		const mods = (await invoke('get_modpack_mods', { id })) as MixedMod[];

		setEditState({
			modpack: {
				id,
				name,
				premade,
				slug: name.toLowerCase().replace(/ /g, '-')
			},
			mods
		});
		startEditing();
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
		><p class="whitespace-nowrap overflow-hidden text-ellipsis w-80">
			{name}
		</p>
		<Fa icon={faCaretRight} class="text-3xl" />
	</a>
	{#if premade}
		<button
			on:click|stopPropagation={viewModpack}
			class="flex flex-col items-center justify-center gap-1 h-14 w-14 bg-indigo-800 hover:bg-indigo-900 transition duration-300 rounded-md shadow-2xl"
			><Fa class="text-lg" icon={faEye} />
			<p class="-mb-1 text-sm">View</p></button
		>
	{:else}
		<button
			on:click|stopPropagation={editModpack}
			class="flex flex-col items-center justify-center gap-1 h-14 w-14 bg-sky-800 hover:bg-sky-900 transition duration-300 rounded-md shadow-2xl"
			><Fa class="text-lg" icon={faPen} />
			<p class="-mb-1 text-sm">Edit</p></button
		>
	{/if}
</div>
