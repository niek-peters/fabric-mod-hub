<script lang="ts">
	import Fa from 'svelte-fa';
	import { faDownload, faUpRightFromSquare } from '@fortawesome/free-solid-svg-icons';

	export let id: number;
	export let name: string;
</script>

<div class="flex gap-4">
	<h3 class="flex-grow bg-zinc-700/20 p-4 rounded-md shadow-2xl text-2xl">{name}</h3>
	<section class="flex gap-4">
		<a
			href="/"
			class="flex flex-col items-center justify-center gap-1 h-16 w-16 bg-creeper/70 hover:bg-creeper/50 long-transition rounded-md shadow-2xl"
			><Fa class="text-lg" icon={faDownload} />
			<p>Install</p></a
		>
		<a
			href="/packs/{id}"
			class="flex flex-col items-center justify-center gap-1 h-16 w-16 bg-indigo-800 hover:bg-indigo-900 long-transition rounded-md shadow-2xl"
			><Fa class="text-lg" icon={faUpRightFromSquare} />
			<p>View</p></a
		>
	</section>
</div>
