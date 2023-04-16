<script lang="ts">
	import { open } from '@tauri-apps/api/shell';
	import { onMount } from 'svelte';
	import { fly } from 'svelte/transition';

	import Fa from 'svelte-fa';
	import { faCaretLeft, faCube, faUpRightFromSquare } from '@fortawesome/free-solid-svg-icons';

	import { viewing, viewState, stopViewing } from '$stores/viewState';
	onMount(() => {
		window.addEventListener('click', () => {
			stopViewing();
		});
	});
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<section
	in:fly={{ y: -50, duration: 250 }}
	out:fly={{ y: -50, duration: 250 }}
	class="absolute w-full h-full p-4 {$viewing ? 'pointer-events-auto' : 'pointer-events-none'}"
>
	<div
		on:click|stopPropagation
		class="relative flex flex-col h-full p-4 gap-4 bg-zinc-700 shadow-2xl rounded-md"
	>
		<div class="flex justify-between items-center">
			<h2 class="text-2xl">View modpack</h2>
			<div class="flex gap-8">
				<button
					on:click={stopViewing}
					class="flex gap-2 items-center text-zinc-300 hover:text-zinc-400 transition duration-300"
				>
					<Fa icon={faCaretLeft} class="text-2xl" />
					Close
				</button>
			</div>
		</div>
		<div class="flex flex-col gap-4 h-full w-full">
			<p class="p-2 rounded-md bg-zinc-600 placeholder-slate-200 text-xl">
				{$viewState.modpack.name}
			</p>
			<div class="list overflow-y-auto -ml-4 pl-4 {$viewState.mods.length > 7 ? 'pr-2' : ''}">
				<div class="list-child flex flex-col w-full gap-2 bg-zinc-800/20 rounded-md p-2">
					{#if $viewState.mods.length === 0}
						<p class="text-lg my-auto text-center text-zinc-200">
							This modpack doesn't have any mods
						</p>
					{/if}
					{#each $viewState.mods as mod}
						{@const iconUrl = `https://cdn.modrinth.com/data/${mod.project_id}/icon.png`}
						{@const iconId = `icon2-${mod.project_id}`}
						<button
							on:click|stopPropagation={() => open(mod.page_url)}
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
							<Fa icon={faUpRightFromSquare} class="ml-auto mx-2 text-lg" />
						</button>
					{/each}
				</div>
			</div>
		</div>
	</div>
</section>

<style lang="scss">
	.list {
		height: 29rem;

		.list-child {
			min-height: 29rem;
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
