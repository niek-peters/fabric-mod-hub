<script>
	import '$src/app.scss';
	import { onMount } from 'svelte';

	onMount(() => {
		// if (document) document.addEventListener('contextmenu', (event) => event.preventDefault());
	});
</script>

<slot />
