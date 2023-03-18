<script lang="ts">
	import Fa from 'svelte-fa';
	import { faPlus } from '@fortawesome/free-solid-svg-icons';

	import Line from '$components/Line.svelte';
	import VerLine from '$components/VerLine.svelte';
	import Transition from '$components/install/Transition.svelte';
	import ModButton from '$components/install/ModButton.svelte';

	import type { LayoutData } from './$types';
	export let data: LayoutData;

	const premade = data.modpacks.filter((modpack) => modpack.premade);
	const custom = data.modpacks.filter((modpack) => !modpack.premade);
</script>

<div class="flex gap-4 h-full">
	<div class="flex flex-col w-3/5 gap-4">
		<section class="flex flex-col gap-4">
			<h1 class="text-xl">Pre-made modpacks</h1>
			{#each premade as modpack}
				<ModButton id={modpack.id} name={modpack.name} />
			{/each}
		</section>
		<Line />
		<section class="flex flex-col items-center gap-4">
			<div class="flex justify-between w-full">
				<h1 class="text-xl">Your modpacks</h1>
				<a
					href="/install/add"
					class="flex gap-2 items-center bg-creeper/80 hover:bg-creeper/60 transition duration-300 px-2 py-1 rounded-md"
					><Fa icon={faPlus} /> Add new
				</a>
			</div>
			{#if custom.length}
				{#each custom as modpack}
					<ModButton id={modpack.id} name={modpack.name} />
				{/each}
			{:else}
				<p class="flex items-center text-zinc-300 text-center text-lg w-4/5 h-24">
					It seems like you haven't created any modpacks yet
				</p>
			{/if}
		</section>
	</div>
	<VerLine />
	<Transition url={data.url}>
		<slot />
	</Transition>
</div>
