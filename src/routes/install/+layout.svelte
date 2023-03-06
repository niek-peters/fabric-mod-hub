<script>
	import VerLine from '$components/VerLine.svelte';
	import ModButton from '$components/install/ModButton.svelte';
</script>

<div class="flex gap-4 h-full">
	<div class="flex flex-col w-3/5 gap-4">
		<h1 class="text-xl">Pre-made modpacks</h1>
		<ModButton id={1} name="Optimizations" />
		<ModButton id={2} name="Multiplayer" />
		<ModButton id={3} name="QoL" />
	</div>
	<VerLine />
	<div class="flex flex-col flex-grow gap-4">
		<slot />
	</div>
</div>
