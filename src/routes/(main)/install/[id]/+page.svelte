<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	import Fa from 'svelte-fa';
	import { faCaretRight, faDownload } from '@fortawesome/free-solid-svg-icons';
	import toast from 'svelte-french-toast';

	import type { PageData } from './$types';
	import { setModpackJoins } from '$stores/modpackJoins';
	export let data: PageData;

	$: selected = '';

	async function install() {
		await invoke('install_modpack_version', { id: data.id, gameVersion: selected });

		const res = await invoke('get_all_modpack_joins');

		// Check if res is an array of modpackjoins
		if (Array.isArray(res)) {
			setModpackJoins(res);
		}

		data.game_versions = data.game_versions.filter((version) => version !== selected);
		(document.getElementById(selected) as HTMLInputElement).checked = false;
		selected = '';
	}
</script>

<section class="flex flex-col h-full gap-4">
	<h2 class="title text-xl whitespace-nowrap overflow-hidden text-ellipsis">
		{data.modpack.name}
	</h2>
	<label class="text-zinc-300 text-lg" for="version">Select a Minecraft version</label>
	<div class="list overflow-y-auto -ml-4 pl-4 {data.game_versions.length > 9 ? 'pr-2' : ''}">
		<div class="flex flex-col gap-2 bg-zinc-900/20 rounded-md p-2">
			{#each data.game_versions as game_version}
				<div
					class="relative flex items-center w-full transition duration-300 rounded-md cursor-pointer {selected ===
					game_version
						? 'bg-zinc-700/50'
						: 'hover:bg-zinc-700/30'}"
				>
					<label class="flex w-full gap-4 px-4 py-2 cursor-pointer" for={game_version}
						>{game_version}</label
					>
					<Fa
						class="absolute text-3xl shadow-2xl text-creeper transition-all duration-300 {selected ===
						game_version
							? '-left-2 opacity-100'
							: '-left-6 opacity-0'}"
						icon={faCaretRight}
					/>
					<input
						type="radio"
						class="absolute opacity-0 w-full h-full cursor-pointer"
						value={game_version}
						id={game_version}
						name="game_version"
						on:input={() => {
							selected = game_version;
						}}
					/>
				</div>
			{/each}
		</div>
	</div>
	<div class="mt-auto flex flex-col gap-2">
		{#if selected}
			<p class="text-zinc-300 text-lg w-64 whitespace-nowrap overflow-hidden text-ellipsis">
				Selected version: {selected}
			</p>
		{:else}
			<p class="text-zinc-300 text-lg">Select a version to install</p>
		{/if}
		<button
			on:click={() => {
				toast.promise(
					install(),
					{
						loading: 'Installing modpack',
						success: 'Installed modpack',
						error: 'Failed to install modpack'
					},
					{
						style: 'background-color: #52525b; color: #e4e4e7; border-radius: 0.375rem;'
					}
				);
			}}
			disabled={!selected}
			class="flex items-center justify-center px-4 py-2 rounded-md gap-2 text-lg  transition duration-300 {selected
				? 'hover:bg-creeper/60 bg-creeper/80 transition duration-300'
				: 'bg-creeper/50 cursor-not-allowed'}}"
			><Fa icon={faDownload} class="text-xl" />Install</button
		>
	</div>
</section>

<style lang="scss">
	.title {
		width: 17rem;
	}

	.list {
		max-height: 28rem;

		/* width */
		&::-webkit-scrollbar {
			width: 0.5rem;
		}

		/* Track */
		&::-webkit-scrollbar-track {
			background-color: rgb(63 63 70 / 0.5);
			border-radius: 0.375rem /* 6px */;
		}

		/* Handle */
		&::-webkit-scrollbar-thumb {
			background-color: rgb(82 82 91 / 0.7);
			transition: all 300ms cubic-bezier(0.4, 0, 0.2, 1);
			border-radius: 0.375rem /* 6px */;

			&:hover {
				background-color: rgb(82 82 91 / 1);
			}
		}
	}
</style>
