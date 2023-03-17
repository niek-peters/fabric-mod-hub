<script lang="ts">
	import Fa from 'svelte-fa';
	import { faCaretRight, faUpRightFromSquare } from '@fortawesome/free-solid-svg-icons';

	import { page } from '$app/stores';

	export let id: number;
	export let name: string;
</script>

<div class="flex gap-4">
	<a
		class="flex justify-between items-center flex-grow transition duration-300 p-4 rounded-md shadow-2xl text-2xl {parseInt(
			$page.params.id
		) === id
			? 'bg-zinc-700/80'
			: 'bg-zinc-700/20 hover:bg-zinc-700/50'}"
		href="/install/{id}"
		>{name}
		<Fa icon={faCaretRight} class="text-3xl" />
	</a>
	<section class="flex gap-4">
		<a
			href="/packs/{id}"
			class="flex flex-col items-center justify-center gap-1 h-16 w-16 bg-indigo-800 hover:bg-indigo-900 transition duration-300 rounded-md shadow-2xl"
			><Fa class="text-lg" icon={faUpRightFromSquare} />
			<p>Info</p></a
		>
	</section>
</div>
