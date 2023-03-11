<script lang="ts">
	import { open } from '@tauri-apps/api/shell';

	import Fa from 'svelte-fa';
	import { faMinus, faUpRightFromSquare } from '@fortawesome/free-solid-svg-icons';

	import type { PageData } from './$types';
	export let data: PageData;
</script>

{#if data.modpack}
	<div class="flex flex-col gap-8 w-full h-full">
		<div class="flex flex-col gap-4">
			<div class="flex gap-4 items-center">
				<h1 class="text-4xl">{data.modpack.name}</h1>
				<Fa icon={faMinus} class="text-lg text-slate-200" />
				<p class="text-xl text-slate-200">{data.modpack.game_version}</p>
			</div>
			<div class="flex gap-4">
				{#if data.modpack.loaded}
					<span class="flex items-center bg-creeper/80 px-4 py-px rounded-full text-md">Loaded</span
					>
				{/if}
				{#if data.modpack.premade}
					<span class="flex items-center bg-sky-900 px-4 py-px rounded-full text-md">Pre-made</span>
				{/if}
			</div>
		</div>
		<div class="flex flex-col flex-grow gap-2">
			<h2 class="text-2xl">Included mods</h2>
			<div class="flex flex-col flex-grow gap-2 bg-zinc-900/20 p-4 rounded-md">
				{#each data.mods as mod}
					<button
						class="flex w-fit gap-2 items-center text-zinc-200 hover:text-zinc-400 long-transition"
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
				class="flex flex-grow items-center justify-center px-4 py-2 rounded-md  long-transition {data
					.modpack.loaded
					? 'bg-fuchsia-800 hover:bg-fuchsia-900'
					: 'bg-creeper/80 hover:bg-creeper/60'}">{data.modpack.loaded ? 'Unload' : 'Load'}</button
			>
			<button
				class="flex flex-grow items-center justify-center px-4 py-2 bg-rose-800 rounded-md hover:bg-rose-900 long-transition"
				>Uninstall</button
			>
		</div>
	</div>
{/if}
