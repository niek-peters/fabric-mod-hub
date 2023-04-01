<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	import Fa from 'svelte-fa';
	import { faMagnifyingGlass } from '@fortawesome/free-solid-svg-icons';

	let searchEl: HTMLInputElement;

	let typing = false;
	let results: NewMod[] = [];

	async function search() {
		const res = await invoke('search', { query: searchEl.value });

		// Check if res is an array of NewMods
		if (Array.isArray(res)) {
			results = res;
		}
	}
</script>

<section class="flex flex-col h-full gap-4">
	<h2 class="text-xl">Create a modpack</h2>
	<form class="flex gap-2" on:submit|preventDefault={search}>
		<button
			type="button"
			class="flex py-2 px-4 gap-4 rounded-full items-center cursor-text transition duration-300 {typing
				? 'bg-zinc-700'
				: 'bg-zinc-700/50'}"
			on:click={() => {
				searchEl.focus();
			}}
		>
			<input
				class="w-48 bg-transparent placeholder-slate-200"
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
	<div class="list overflow-y-auto -ml-4 pl-4">
		<div class="flex flex-col gap-2 bg-zinc-900/20 rounded-md p-2">
			{#each results as result}
				<div
					class="relative flex items-center w-full transition duration-300 rounded-md cursor-pointer"
				>
					{result.name}
				</div>
			{/each}
		</div>
	</div>
</section>

<style lang="scss">
	.list {
		max-height: 28rem;

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
