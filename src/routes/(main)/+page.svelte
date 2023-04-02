<script lang="ts">
	import { open } from '@tauri-apps/api/shell';

	import Fa from 'svelte-fa';
	import { faMinus, faUpRightFromSquare } from '@fortawesome/free-solid-svg-icons';
	import { faGithub } from '@fortawesome/free-brands-svg-icons';
	import { modpackJoins, unload } from '$stores/modpackJoins';

	$: loadedModpack = $modpackJoins.find((join) => join.loaded);
</script>

<div class="flex flex-col flex-grow w-full gap-8">
	<section class="relative flex items-center justify-center h-72 py-8 overflow-hidden rounded-md">
		<img class="absolute brightness-75 z-10" src="background-1.webp" alt="Minecraft background" />
		<img class="h-full brightness-75 shadow-2xl z-20" src="favicon.svg" alt="Fabric Mod Hub icon" />
		<h1 class="absolute text-5xl text-center text-shadow z-30">Welcome to Fabric Mod Hub</h1>
	</section>
	{#if loadedModpack}
		<article class="flex flex-col gap-4 w-full h-fit">
			<h2 class="text-xl">Currently loaded modpack</h2>
			<div class="flex gap-4">
				<section class="flex flex-col gap-4 bg-zinc-700/20 p-4 rounded-md flex-grow shadow-2xl">
					<div class="flex justify-between">
						<h3 class="text-2xl">{loadedModpack.name}</h3>
						<span class="font-semibold">{loadedModpack.game_version}</span>
					</div>
					<section class="flex items-center">
						{#if loadedModpack.premade}
							<span class="flex items-center bg-sky-900 px-4 py-px rounded-full text-md"
								>Pre-made</span
							>
						{/if}
					</section>
				</section>
				<section class="flex flex-col gap-4 w-1/3">
					<a
						href="/packs/{loadedModpack.id}"
						class="flex items-center gap-4 px-4 flex-grow bg-indigo-800 hover:bg-indigo-900 transition duration-300 rounded-md shadow-2xl"
						><Fa class="text-lg" icon={faUpRightFromSquare} />
						<p>View</p></a
					>
					<button
						on:click={unload}
						class="flex items-center gap-4 px-4 flex-grow bg-fuchsia-800 hover:bg-fuchsia-900 transition duration-300 rounded-md shadow-2xl"
						><Fa class="text-lg" icon={faMinus} />
						<p>Unload</p></button
					>
				</section>
			</div>
		</article>
	{/if}
	<section class="flex mt-auto justify-end h-fit">
		<button
			class="flex gap-2 items-center hover:text-slate-300 transition duration-300"
			on:click={() => open('https://github.com/MrValk/fabric-mod-hub')}
		>
			<Fa class="text-2xl" icon={faGithub} />
			<p>View the project on GitHub</p>
			<Fa class="text-lg" icon={faUpRightFromSquare} />
		</button>
	</section>
</div>

<style>
	.text-shadow {
		text-shadow: 0 15px 30px rgba(0, 0, 0, 0.6), 0 5px 15px rgba(0, 0, 0, 0.5);
	}
</style>
