<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<div class="w-full h-full">
	<p>Pack {data.id}</p>
	<p>Mods:</p>
	{#each data.mods as mod}
		<p>{mod.name}</p>
	{/each}
</div>
