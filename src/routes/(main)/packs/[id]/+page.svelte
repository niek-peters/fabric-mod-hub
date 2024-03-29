<script lang="ts">
	import { open } from '@tauri-apps/api/shell';
	import { invoke } from '@tauri-apps/api/tauri';

	import Fa from 'svelte-fa';
	import { faMinus, faUpRightFromSquare, faPen, faTrash } from '@fortawesome/free-solid-svg-icons';
	import toast from 'svelte-french-toast';

	import type { PageData } from './$types';
	import { modpackJoins, loadFromVersionId, unload, remove } from '$stores/modpackJoins';
	import { goto } from '$app/navigation';
	import { editingPack, startEditing } from '$src/lib/stores/editPackState';
	import EditOverlay from '$src/lib/components/packs/EditOverlay.svelte';
	export let data: PageData;

	$: modpackJoin = $modpackJoins.find((join) => join.id === data.id);

	async function uninstall() {
		if (!modpackJoin) return;

		if (modpackJoin.loaded) {
			unload();
		}

		await invoke('uninstall_modpack_version', { id: modpackJoin.id });

		await remove(modpackJoin.id);

		toast.success('Uninstalled modpack', {
			style: 'background-color: #52525b; color: #e4e4e7; border-radius: 0.375rem;'
		});

		await goto('/');
	}
</script>

{#if modpackJoin}
	<div
		class="relative flex flex-col gap-8 w-full h-full {$editingPack ? 'pointer-events-none' : ''}"
	>
		<div class="flex flex-col gap-4">
			<div class="flex justify-between">
				<div class="flex gap-4 items-center">
					<h1 class="title text-4xl whitespace-nowrap overflow-hidden text-ellipsis">
						{modpackJoin.custom_name || modpackJoin.name}
					</h1>
					<Fa icon={faMinus} class="text-lg text-slate-200" />
					<p class="text-xl text-slate-200 whitespace-nowrap overflow-hidden text-ellipsis w-36">
						{modpackJoin.game_version}
					</p>
				</div>
				<button
					on:click={startEditing}
					class="flex items-center gap-2 text-indigo-400 hover:text-indigo-500 transition duration-300"
					><Fa class="text-lg" icon={faPen} />Edit modpack</button
				>
			</div>
			<div class="relative flex gap-4 w-full h-8">
				<span
					class="absolute flex items-center justify-center bg-creeper/80 h-8 rounded-full text-md transition-all duration-300 {modpackJoin.loaded
						? 'opacity-100 w-24'
						: 'opacity-0 w-0 delay-75'}"
					><p
						class="absolute flex justify-center transition-all duration-300 {modpackJoin.loaded
							? 'opacity-100 w-16 delay-75'
							: 'opacity-0 w-0'}"
					>
						Loaded
					</p></span
				>
				<div
					class="absolute flex gap-4 transition-all duration-300 {modpackJoin.loaded
						? 'left-28'
						: 'left-0 delay-75'}"
				>
					{#if modpackJoin.premade}
						<span class="relative flex items-center bg-sky-900 px-4 h-8 rounded-full text-md"
							>Pre-made</span
						>
					{/if}
				</div>
			</div>
		</div>
		<div class="flex flex-col flex-grow gap-2">
			<h2 class="text-2xl">Included mods</h2>
			<div class="flex flex-col flex-grow gap-2 bg-zinc-900/20 p-4 rounded-md">
				{#each data.mods as mod}
					<button
						class="flex w-fit gap-2 items-center text-zinc-200 hover:text-zinc-400 transition duration-300"
						on:click={() => open(mod.page_url)}
					>
						<p>{mod.name}</p>
						<Fa icon={faUpRightFromSquare} class="text-sm" />
					</button>
				{/each}
			</div>
		</div>
		<div class="flex gap-4">
			<button
				on:click={async () => {
					if (!modpackJoin) return;

					if (modpackJoin.loaded) unload();
					else loadFromVersionId(modpackJoin.id);
				}}
				class="flex w-1/2 items-center justify-center px-4 py-2 rounded-md  transition duration-300 shadow-2xl {modpackJoin.loaded
					? 'bg-fuchsia-800 hover:bg-fuchsia-900'
					: 'bg-creeper/80 hover:bg-creeper/60'}">{modpackJoin.loaded ? 'Unload' : 'Load'}</button
			>
			<button
				on:click={uninstall}
				class="flex w-1/2 items-center justify-center px-4 py-2 bg-rose-800 rounded-md hover:bg-rose-900 transition duration-300 shadow-2xl"
				>Uninstall</button
			>
		</div>
		{#if $editingPack}
			<EditOverlay id={data.id} name={modpackJoin.custom_name || modpackJoin.name} />
		{/if}
	</div>
{/if}

<style>
	.title {
		max-width: 36rem;
	}
</style>
