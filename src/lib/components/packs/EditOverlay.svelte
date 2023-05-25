<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/shell';
	import { onDestroy, onMount } from 'svelte';
	import { fly } from 'svelte/transition';
	import { goto } from '$app/navigation';

	import Fa from 'svelte-fa';
	import {
		faMagnifyingGlass,
		faTrash,
		faCaretLeft,
		faCube,
		faUpRightFromSquare,
		faSave,
		faRotateLeft
	} from '@fortawesome/free-solid-svg-icons';
	import toast from 'svelte-french-toast';

	import {
		editingPack,
		editPackState,
		addFilepath,
		removeFilepath,
		resetEditState,
		stopEditing
	} from '$src/lib/stores/editPackState';
	import VerLine from '../VerLine.svelte';
	import { modpackJoins, remove, setModpackJoins, unload } from '$src/lib/stores/modpackJoins';

	export let id: number;
	export let name: string;
	$editPackState.custom_name = name;
	// export let updateList: (newFilepath: string) => void;
	// export let deleteFunction: (filepath: string) => void = () => {};

	let formEl: HTMLFormElement;

	let originalFilepaths: string[] = [];

	onMount(() => {
		originalFilepaths = structuredClone($editPackState.custom_filepaths);

		setTimeout(() => {
			window.addEventListener('click', stopEditing);
		}, 300);
	});

	onDestroy(() => {
		window.removeEventListener('click', stopEditing);
	});

	async function save() {
		const removedFilepaths = originalFilepaths.filter(
			(filepath) => !$editPackState.custom_filepaths.includes(filepath)
		);
		const newFilepaths = $editPackState.custom_filepaths.filter(
			(filepath) => !originalFilepaths.find((addedFilepath) => addedFilepath === filepath)
		);

		await invoke('update_modpack_version', {
			id,
			customName: $editPackState.custom_name,
			removedFilepaths,
			newFilepaths
		});

		const res = await invoke('get_all_modpack_joins');

		// Check if res is an array of modpackjoins
		if (Array.isArray(res)) {
			setModpackJoins(res);
		}

		// Unload all modpack versions
		unload();

		resetEditState();
		stopEditing();
	}

	function getFileName(filepath: string) {
		// @ts-ignore
		return filepath.split('\\').pop().split('/').pop();
	}
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<section
	in:fly={{ y: -50, duration: 250 }}
	out:fly={{ y: -50, duration: 250 }}
	class="absolute w-full h-full p-4 {$editingPack ? 'pointer-events-auto' : 'pointer-events-none'}"
>
	<div
		on:click|stopPropagation
		class="relative flex flex-col h-full p-4 gap-4 bg-zinc-700 shadow-2xl rounded-md"
	>
		<div class="flex justify-between items-center">
			<h2 class="text-2xl">Edit modpack</h2>
			<div class="flex gap-8">
				<button
					on:click={stopEditing}
					class="flex gap-2 items-center text-zinc-300 hover:text-zinc-400 transition duration-300"
				>
					<Fa icon={faCaretLeft} class="text-2xl" />
					Close
				</button>
				<button
					on:click={resetEditState}
					class="flex gap-2 items-center text-rose-500 hover:text-rose-600 transition duration-300"
				>
					<Fa icon={faRotateLeft} />
					Revert Changes
				</button>
			</div>
		</div>
		<div class="flex gap-4 h-full w-full">
			<form
				class="flex flex-col flex-grow gap-4 h-full w-1/2"
				on:submit|preventDefault={async () => {
					toast.promise(
						save(),
						{
							loading: 'Saving modpack...',
							success: 'Modpack saved!',
							error: 'Failed to save modpack'
						},
						{
							style: 'background-color: #52525b; color: #e4e4e7; border-radius: 0.375rem;'
						}
					);
				}}
				bind:this={formEl}
			>
				<input
					class="w-full p-2 rounded-md bg-zinc-600 placeholder-slate-200"
					type="text"
					placeholder="Enter your modpack name"
					minlength="1"
					maxlength="32"
					required
					bind:value={$editPackState.custom_name}
				/>
				<div
					class="list overflow-y-auto -ml-4 pl-4 {$editPackState.custom_filepaths.length > 7
						? 'pr-2'
						: ''}"
				>
					<div class="list-child flex flex-col w-full gap-2 bg-zinc-800/20 rounded-md p-2">
						{#if $editPackState.custom_filepaths.length === 0}
							<p class="text-lg my-auto text-center text-zinc-200">
								Add a file using the add button
							</p>
						{/if}
						{#each $editPackState.custom_filepaths as filepath}
							<button
								type="button"
								class="relative flex items-center gap-2 w-full p-2 bg-zinc-600/20 hover:bg-zinc-600/50 transition duration-300 rounded-md cursor-pointer"
							>
								<div
									class="flex items-center justify-center w-8 h-8 rounded-md bg-cover overflow-hidden text-white"
								>
									<Fa icon={faCube} class="hidden text-3xl" />
								</div>
								<p class="w-48 text-left whitespace-nowrap overflow-hidden text-ellipsis">
									{getFileName(filepath)}
								</p>
								<button
									type="button"
									class="ml-auto p-2 text-zinc-400 hover:bg-indigo-900 transition duration-300 rounded-md"
								>
									<Fa icon={faTrash} class="text-lg" />
								</button>
							</button>
						{/each}
					</div>
				</div>
				<div class="relative w-full">
					<button
						type="submit"
						class="flex items-center justify-center w-full gap-2 px-4 py-2 rounded-md transition duration-300 bg-creeper/80 hover:bg-creeper/60"
						><Fa icon={faSave} />Save Modpack</button
					>
				</div>
			</form>
		</div>
	</div>
</section>

<style lang="scss">
	.list {
		height: 25.75rem;

		.list-child {
			min-height: 25.75rem;
		}

		/* width */
		&::-webkit-scrollbar {
			width: 0.5rem;
		}

		/* Track */
		&::-webkit-scrollbar-track {
			background-color: rgb(39 39 42 / 0.2);
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
