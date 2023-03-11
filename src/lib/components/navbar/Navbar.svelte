<script lang="ts">
	import Fa from 'svelte-fa';
	import { faGear, faHome, faMagnifyingGlass, faPlus } from '@fortawesome/free-solid-svg-icons';

	import Line from '$components/Line.svelte';
	import NavButton from '$components/navbar/NavButton.svelte';
	import ModButton from '$components/navbar/ModButton.svelte';

	export let modpacks: ModpackJoin[] | undefined;
</script>

<nav class="flex flex-col w-fit h-full bg-zinc-700/30 p-4">
	<section class="flex flex-col gap-6">
		<NavButton icon={faHome} text="Home" href="/" routeId="/(main)" />
		<Line />
		<div class="flex flex-col gap-4">
			<div class="flex bg-zinc-700/30 py-2 px-4 gap-4 rounded-full items-center">
				<Fa icon={faMagnifyingGlass} />
				<input
					class="bg-transparent placeholder-slate-200"
					type="text"
					placeholder="Search modpacks"
				/>
			</div>
			<section class="flex flex-col gap-4">
				{#if modpacks}
					{#each modpacks as modpack}
						{#if modpack.id}
							<ModButton
								id={modpack.id}
								name={modpack.name}
								version={modpack.game_version}
								loaded={modpack.loaded}
							/>
						{/if}
					{/each}
				{/if}
			</section>
		</div>
		<Line />
		<NavButton icon={faPlus} text="Install modpack" href="/install" routeId="/(main)/install" />
		<Line />
		<NavButton icon={faGear} text="Settings" href="/settings" routeId="/(main)/settings" />
	</section>
	<section class="mt-auto flex justify-center">
		<p>&copy; Niek Peters 2023 - v0.0.1</p>
	</section>
</nav>
