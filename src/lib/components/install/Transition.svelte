<script lang="ts">
	import { navigating } from '$app/stores';
	import { fly } from 'svelte/transition';

	import { Moon } from 'svelte-loading-spinners';

	export let url: string;
</script>

<div class="grid flex-grow">
	{#key url}
		<main
			class="flex flex-col w-full h-full"
			in:fly={{ y: -50, duration: 250, delay: 300 }}
			out:fly={{ y: -50, duration: 250 }}
		>
			{#if $navigating}
				<div class="flex flex-col gap-4 items-center justify-center flex-grow">
					<Moon size="75" color="rgb(60 175 44)" />
					<p class="text-lg">Loading<span /></p>
				</div>
			{:else}
				<slot />
			{/if}
		</main>
	{/key}
</div>

<style>
	main {
		grid-area: 1/1/2/2;
	}

	p span:before {
		animation: dots 1s linear infinite;
		content: '';
	}

	@keyframes dots {
		0%,
		20% {
			content: '.';
		}
		40% {
			content: '..';
		}
		60% {
			content: '...';
		}
		90%,
		100% {
			content: '';
		}
	}
</style>
