<script lang="ts">
	import { faCaretRight } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';

	export let id: number;
	export let name: string;
	export let version: string;
	export let loaded: boolean = false;
</script>

<a
	href="/packs/{id}"
	class="relative flex items-center justify-between gap-4 py-2 px-4 rounded-md bg-zinc-700/50"
>
	<p>{name}</p>
	<p class="text-sm text-slate-200">{version}</p>

	<Fa
		class="absolute text-3xl  -left-2 shadow-2xl {loaded ? 'text-creeper' : 'text-transparent'}"
		icon={faCaretRight}
	/>
</a>
