<script lang="ts">
	import type { IconDefinition } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';

	export let icon: IconDefinition;
	export let text: string;
	export let href: string;
	export let routeId: string = href;

	import { page } from '$app/stores';
	import { clearSearch } from '$stores/search';
</script>

<a
	{href}
	on:click={clearSearch}
	class="flex items-center gap-4 py-2 px-4 rounded-md transition duration-300 {(routeId !==
		'/(main)' &&
		$page.route.id?.includes(routeId)) ||
	$page.route.id === routeId
		? 'bg-zinc-600/80'
		: 'bg-zinc-700/50 hover:bg-zinc-700'}"
>
	<Fa {icon} />
	<p>{text}</p>
</a>
