<script lang="ts">
	import '$src/app.css';

	import Navbar from '$components/Navbar.svelte';
</script>

<div class="flex w-screen h-screen bg-zinc-800 text-slate-50">
	<Navbar />
	<main class="flex flex-col flex-grow h-screen p-4">
		<slot />
	</main>
</div>
