<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/shell';
	import { onMount } from 'svelte';
	import { fly } from 'svelte/transition';

	import Fa from 'svelte-fa';
	import {
		faMagnifyingGlass,
		faTrash,
		faCaretLeft,
		faCube,
		faUpRightFromSquare,
		faPlus
	} from '@fortawesome/free-solid-svg-icons';

	import {
		adding,
		addMod,
		addState,
		removeMod,
		resetAddState,
		setTitle,
		stopAdding
	} from '$stores/addState';
	import VerLine from '../VerLine.svelte';

	let searchEl: HTMLInputElement;
	let formEl: HTMLFormElement;

	let typing = false;
	let results: NewMod[] = [];
	let filteredResults: NewMod[] = [];
	$: {
		filteredResults = results.filter((mod) => {
			return !$addState.mods.find((addedMod) => addedMod.project_id === mod.project_id);
		});
	}
	let helpHover = false;

	let name: string;
	let noMods = false;
	$: {
		if ($addState.mods.length) noMods = false;
	}

	onMount(() => {
		searchEl.focus();
		search();

		name = $addState.modpack.name;

		window.addEventListener('click', () => {
			stopAdding();
		});
	});

	async function search() {
		const res = await invoke('search', { query: searchEl.value });

		// Check if res is an array of NewMods
		if (Array.isArray(res)) {
			results = res;
		}
	}
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<section
	in:fly={{ y: -50, duration: 250 }}
	out:fly={{ y: -50, duration: 250 }}
	class="absolute w-full h-full p-4 {$adding ? 'pointer-events-auto' : 'pointer-events-none'}"
>
	<div
		on:click|stopPropagation
		class="relative flex flex-col h-full p-4 gap-4 bg-zinc-700 shadow-2xl rounded-md"
	>
		<div class="flex justify-between items-center">
			<h2 class="text-2xl">Create a modpack</h2>
			<div class="flex gap-8">
				<button
					on:click={stopAdding}
					class="flex gap-2 items-center text-zinc-300 hover:text-zinc-400 transition duration-300"
				>
					<Fa icon={faCaretLeft} class="text-2xl" />
					Close
				</button>
				<button
					on:click={() => {
						name = '';
						resetAddState();
						searchEl.focus();
					}}
					class="flex gap-2 items-center text-rose-500 hover:text-rose-600 transition duration-300"
				>
					<Fa icon={faTrash} />
					Reset
				</button>
			</div>
		</div>
		<div class="flex gap-4 h-full w-full">
			<section class="flex flex-col gap-4 w-1/2 h-full">
				<form class="flex gap-2" on:submit|preventDefault={search}>
					<button
						type="button"
						class="flex w-full py-2 px-4 gap-4 rounded-full items-center cursor-text transition duration-300 {typing
							? 'bg-zinc-600'
							: 'bg-zinc-600/50'}"
						on:click={() => {
							searchEl.focus();
						}}
					>
						<input
							class="bg-transparent placeholder-slate-200"
							type="text"
							placeholder="Search for mods"
							on:focusin={() => (typing = true)}
							on:focusout={() => (typing = false)}
							bind:this={searchEl}
						/>
					</button>
					<button
						type="submit"
						class="flex justify-center items-center bg-creeper/80 hover:bg-creeper/60 transition duration-300 aspect-square h-full rounded-full"
					>
						<Fa icon={faMagnifyingGlass} />
					</button>
				</form>
				<div class="list overflow-y-auto -ml-4 pl-4 {results.length > 7 ? 'pr-2' : ''}">
					<div class="list-child flex flex-col w-full gap-2 bg-zinc-800/20 rounded-md p-2">
						<!-- Loop over results. Filter out all results with project_ids that are in the $addState.mods list -->
						{#if filteredResults.length === 0}
							<p class="text-lg my-auto text-center text-zinc-200">No (not-selected) mods found</p>
						{/if}
						{#each filteredResults as result}
							{@const iconUrl = `https://cdn.modrinth.com/data/${result.project_id}/icon.png`}
							{@const iconId = `icon-${result.project_id}`}
							<button
								on:click={() => addMod(result)}
								class="relative flex items-center gap-2 w-full p-2 bg-zinc-600/20 hover:bg-zinc-600/50 transition duration-300 rounded-md cursor-pointer"
							>
								<div
									class="flex items-center justify-center w-8 h-8 rounded-md bg-cover overflow-hidden text-white"
									style="background-image: url({iconUrl})"
								>
									<img
										class="hidden"
										src={iconUrl}
										alt=""
										on:error={function () {
											document.getElementById(iconId)?.classList.remove('hidden');
										}}
										on:load={function () {
											document.getElementById(iconId)?.classList.add('hidden');
										}}
									/>
									<Fa icon={faCube} id={iconId} class="hidden text-3xl" />
								</div>
								<p class="w-48 text-left whitespace-nowrap overflow-hidden text-ellipsis">
									{result.name}
								</p>
								<button
									on:click|stopPropagation={() => open(result.page_url)}
									class="ml-auto p-2 text-zinc-400 hover:bg-indigo-900 transition duration-300 rounded-md"
								>
									<Fa icon={faUpRightFromSquare} class="text-lg" />
								</button>
							</button>
						{/each}
					</div>
				</div>
				<div
					class="relative mt-auto flex items-center justify-center w-full py-2 text-indigo-400 hover:text-indigo-500 transition duration-300"
				>
					{#if helpHover}
						<span
							in:fly={{ y: 20, duration: 250 }}
							out:fly={{ y: 20, duration: 250 }}
							class="absolute flex flex-col gap-2 bg-white text-zinc-700 bottom-14 p-2 rounded-md"
						>
							<p>You can manually add the files after installing the modpack</p>
							<img
								class="rounded-md"
								src="/polish-cow.gif"
								alt="Navigate to it using the menu on the left. Click the 'Add external mod' button to add a file."
							/>
						</span>
					{/if}
					<p on:mouseenter={() => (helpHover = true)} on:mouseleave={() => (helpHover = false)}>
						Can't find a mod?
					</p>
				</div>
			</section>
			<VerLine color="bg-zinc-600" />
			<form
				class="flex flex-col flex-grow gap-4 h-full w-1/2"
				on:submit|preventDefault={() => {
					if (!$addState.mods.length && !noMods) noMods = true;
					else formEl.submit();
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
					bind:value={name}
					on:input={() => {
						setTitle(name);
					}}
				/>
				<div class="list overflow-y-auto -ml-4 pl-4 {$addState.mods.length > 7 ? 'pr-2' : ''}">
					<div class="list-child flex flex-col w-full gap-2 bg-zinc-800/20 rounded-md p-2">
						{#if $addState.mods.length === 0}
							<p class="text-lg my-auto text-center text-zinc-200">
								Add a mod using the search bar
							</p>
						{/if}
						{#each $addState.mods as mod}
							{@const iconUrl = `https://cdn.modrinth.com/data/${mod.project_id}/icon.png`}
							{@const iconId = `icon2-${mod.project_id}`}
							<button
								type="button"
								on:click={() => removeMod(mod)}
								class="relative flex items-center gap-2 w-full p-2 bg-zinc-600/20 hover:bg-zinc-600/50 transition duration-300 rounded-md cursor-pointer"
							>
								<div
									class="flex items-center justify-center w-8 h-8 rounded-md bg-cover overflow-hidden text-white"
									style="background-image: url({iconUrl})"
								>
									<img
										class="hidden"
										src={iconUrl}
										alt=""
										on:error={function () {
											document.getElementById(iconId)?.classList.remove('hidden');
										}}
										on:load={function () {
											document.getElementById(iconId)?.classList.add('hidden');
										}}
									/>
									<Fa icon={faCube} id={iconId} class="hidden text-3xl" />
								</div>
								<p class="w-48 text-left whitespace-nowrap overflow-hidden text-ellipsis">
									{mod.name}
								</p>
								<button
									type="button"
									on:click|stopPropagation={() => open(mod.page_url)}
									class="ml-auto p-2 text-zinc-400 hover:bg-indigo-900 transition duration-300 rounded-md"
								>
									<Fa icon={faUpRightFromSquare} class="text-lg" />
								</button>
							</button>
						{/each}
					</div>
				</div>
				<div class="relative w-full">
					{#if noMods}
						<span
							in:fly={{ y: 20, duration: 250 }}
							out:fly={{ y: 20, duration: 250 }}
							class="absolute flex flex-col w-full gap-2 bg-white text-zinc-700 bottom-14 p-2 rounded-md"
						>
							<p class="font-bold">Are you sure you want to create an empty modpack?</p>
							<p>You can manually add the files after installing the modpack</p>
							<img
								class="rounded-md"
								src="/polish-cow.gif"
								alt="Navigate to it using the menu on the left. Click the 'Add external mod' button to add a file."
							/>
						</span>
					{/if}
					<button
						type="submit"
						class="flex items-center justify-center w-full gap-2 px-4 py-2 rounded-md transition duration-300 {noMods
							? 'bg-rose-800 hover:bg-rose-900'
							: 'bg-creeper/80 hover:bg-creeper/60'}"
						><Fa icon={faPlus} />{noMods ? 'Create empty modpack' : 'Add modpack'}</button
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
