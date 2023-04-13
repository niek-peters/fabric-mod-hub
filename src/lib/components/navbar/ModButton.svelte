<script lang="ts">
	import { page } from '$app/stores';
	import { faCaretRight } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';

	export let id: number;
	export let name: string;
	export let version: string;
	export let loaded: boolean = false;

	$: href = `/packs/${id}`;
	$: routeId = `/(main)/packs/${id}`;
</script>

<a
	{href}
	class="relative flex items-center justify-between gap-4 py-2 px-4 rounded-md transition duration-300 {$page
		.route.id && $page.route.id.replace('[id]', $page.params.id) === routeId
		? 'bg-zinc-600/80'
		: 'bg-zinc-700/50 hover:bg-zinc-700'}"
>
	<p class="w-40 whitespace-nowrap overflow-hidden text-ellipsis">{name}</p>
	<p class="text-sm text-slate-200 text-right w-16 whitespace-nowrap overflow-hidden text-ellipsis">
		{version}
	</p>

	<Fa
		class="absolute text-3xl shadow-2xl text-creeper transition-all duration-300 {loaded
			? '-left-2 opacity-100'
			: '-left-6 opacity-0'}"
		icon={faCaretRight}
	/>
</a>
