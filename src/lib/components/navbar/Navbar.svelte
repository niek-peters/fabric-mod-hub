<script lang="ts">
	import { onMount } from 'svelte';

	import Fa from 'svelte-fa';
	import { faGear, faHome, faMagnifyingGlass, faPlus } from '@fortawesome/free-solid-svg-icons';

	import Line from '$components/Line.svelte';
	import NavButton from '$components/navbar/NavButton.svelte';
	import ModButton from '$components/navbar/ModButton.svelte';

	import { search } from '$stores/search';
	import { foundMcDir } from '$src/lib/stores/foundMcDir';

	export let modpacks: ModpackJoin[];

	let searchEl: HTMLInputElement;

	let typing = false;

	onMount(() => {
		searchEl.focus();
	});

	$: filteredModpacks = modpacks
		.filter(
			(modpack) =>
				removeWhitespace(modpack.name.toLowerCase()).includes(
					removeWhitespace($search.toLowerCase())
				) ||
				removeWhitespace(modpack.game_version.toLowerCase()).includes(
					removeWhitespace($search.toLowerCase())
				)
		)
		.sort(
			// Sort in reverse order
			(a, b) => b.id - a.id
		);

	function removeWhitespace(str: string) {
		return str.replace(/\s/g, '');
	}
</script>

<nav
	class="flex flex-col w-fit h-full bg-zinc-700/30 p-4 mc-dir {!$foundMcDir
		? 'no-mc-dir pointer-events-none'
		: 'pointer-events-auto'}"
>
	<section class="flex flex-col gap-6">
		<NavButton icon={faHome} text="Home" href="/" routeId="/(main)" />
		<Line />
		<div class="flex flex-col gap-4">
			<button
				class="flex py-2 px-4 gap-4 rounded-full items-center cursor-text transition duration-300 {typing
					? 'bg-zinc-700'
					: 'bg-zinc-700/50'}"
				on:click={() => {
					searchEl.focus();
				}}
			>
				<Fa icon={faMagnifyingGlass} />
				<input
					class="bg-transparent placeholder-slate-200"
					type="text"
					placeholder="Search modpacks"
					on:focusin={() => (typing = true)}
					on:focusout={() => (typing = false)}
					bind:value={$search}
					bind:this={searchEl}
				/>
			</button>
			<section
				class="modpacks flex flex-col gap-4 -ml-4 pl-4 overflow-y-auto {filteredModpacks.length > 5
					? 'pr-2'
					: ''}"
			>
				{#if filteredModpacks.length}
					{#each filteredModpacks as modpack}
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

<style lang="scss">
	.mc-dir {
		filter: blur(0px);
		transition: filter 300ms cubic-bezier(0.4, 0, 0.2, 1);

		&.no-mc-dir {
			filter: blur(2px);
		}
	}

	.modpacks {
		height: 16.5rem;

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
