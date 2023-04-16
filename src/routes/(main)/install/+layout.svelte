<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';

	import Fa from 'svelte-fa';
	import { faPlus } from '@fortawesome/free-solid-svg-icons';
	import toast from 'svelte-french-toast';

	import Line from '$components/Line.svelte';
	import VerLine from '$components/VerLine.svelte';
	import Transition from '$components/install/Transition.svelte';
	import ModpackButton from '$components/install/ModpackButton.svelte';
	import AddOverlay from '$components/install/AddOverlay.svelte';
	import EditOverlay from '$components/install/EditOverlay.svelte';
	import ViewOverlay from '$components/install/ViewOverlay.svelte';

	import { adding, startAdding } from '$stores/addState';
	import { editing } from '$stores/editState';
	import { viewing } from '$stores/viewState';

	import type { LayoutData } from './$types';
	export let data: LayoutData;

	const premade = data.modpacks.filter((modpack) => modpack.premade);
	let custom = data.modpacks.filter((modpack) => !modpack.premade);

	function addCustom(newModpack: Modpack) {
		custom.push(newModpack);
		custom = custom;
	}

	function deleteCustom(id: number) {
		custom = custom.filter((modpack) => modpack.id !== id);

		toast('Deleted modpack', {
			icon: '🗑️',
			style: 'background-color: #52525b; color: #e4e4e7; border-radius: 0.375rem;'
		});

		if ($page.params.id === id.toString()) goto('/install');
	}

	function update(updatedModpack: Modpack) {
		custom = custom.map((modpack) => {
			if (modpack.id === updatedModpack.id) return updatedModpack;
			return modpack;
		});
	}
</script>

<div class="relative flex gap-4 h-full {$adding ? 'pointer-events-none' : ''}">
	<div class="flex flex-col w-3/5 flex-shrink-0 gap-4">
		<section class="flex flex-col gap-4">
			<h1 class="text-xl">Pre-made modpacks</h1>
			<div class="premades flex flex-col gap-4 overflow-y-auto {premade.length > 3 ? 'pr-2' : ''}">
				{#each premade as modpack}
					<ModpackButton id={modpack.id} name={modpack.name} />
				{/each}
			</div>
		</section>
		<Line />
		<section class="flex flex-col gap-4">
			<div class="flex justify-between w-full">
				<h1 class="text-xl">Your modpacks</h1>
				<button
					on:click|stopPropagation={startAdding}
					class="flex gap-2 items-center bg-creeper/80 hover:bg-creeper/60 transition duration-300 px-2 py-1 rounded-md"
					><Fa icon={faPlus} /> Add new
				</button>
			</div>
			<div class="customs flex flex-col gap-4 overflow-y-auto {custom.length > 4 ? 'pr-2' : ''}">
				{#if custom.length}
					{#each custom as modpack}
						<ModpackButton id={modpack.id} name={modpack.name} premade={false} />
					{/each}
				{:else}
					<div class="w-full h-24 flex items-center justify-center">
						<p class="text-center text-zinc-300 text-lg w-4/5">
							It seems like you haven't created any modpacks yet
						</p>
					</div>
				{/if}
			</div>
		</section>
	</div>
	<VerLine />
	<div class="flex flex-grow">
		<Transition url={data.url}>
			<slot />
		</Transition>
	</div>
	{#if $adding}
		<AddOverlay updateList={addCustom} />
	{:else if $editing}
		<EditOverlay updateList={update} deleteFunction={deleteCustom} />
	{:else if $viewing}
		<ViewOverlay />
	{/if}
</div>

<style lang="scss">
	.premades,
	.customs {
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

	.premades {
		height: 12.5rem;
	}

	.customs {
		height: 19.45rem;
	}
</style>
