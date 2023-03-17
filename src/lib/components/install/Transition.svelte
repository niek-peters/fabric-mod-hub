<script lang="ts">
	import { fly } from 'svelte/transition';

	export let url: string;
</script>

<div class="grid flex-grow">
	{#key url}
		<main
			class="flex flex-col w-full h-full"
			in:fly={{ y: -50, duration: 250, delay: 300 }}
			out:fly={{ y: -50, duration: 250 }}
		>
			<slot />
		</main>
	{/key}
</div>

<style>
	main {
		grid-area: 1/1/2/2;
	}
</style>
