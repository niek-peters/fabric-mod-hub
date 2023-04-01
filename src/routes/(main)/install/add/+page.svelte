<script lang="ts">
	import Fa from 'svelte-fa';
	import { faMagnifyingGlass } from '@fortawesome/free-solid-svg-icons';

	let typing = false;
</script>

<section class="flex flex-col h-full gap-4">
	<h2 class="text-xl">Create a modpack</h2>
	<div class="flex gap-4 w-full">
		<div
			class="flex py-2 px-4 gap-4 rounded-full items-center transition duration-300 {typing
				? 'bg-zinc-700'
				: 'bg-zinc-700/50'}"
		>
			<Fa icon={faMagnifyingGlass} />
			<input
				class="bg-transparent placeholder-slate-200"
				type="text"
				placeholder="Search for mods"
				on:focusin={() => (typing = true)}
				on:focusout={() => (typing = false)}
			/>
		</div>
		<button> blyat </button>
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
