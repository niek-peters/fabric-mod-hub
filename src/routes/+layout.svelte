<script lang="ts">
	import '$src/app.css';

	import Titlebar from '$components/Titlebar.svelte';
	import Transition from '$components/Transition.svelte';
	import Navbar from '$components/navbar/Navbar.svelte';
	import type { LayoutData } from './$types';

	export let data: LayoutData;
</script>

<Titlebar />
<div class="flex mt-8 w-screen bg-zinc-800 text-slate-50">
	<Navbar />
	<Transition url={data.url}>
		<slot />
	</Transition>
</div>

<style>
	div {
		height: calc(100vh - 2rem);
	}
</style>
