<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';

	import Titlebar from '$components/Titlebar.svelte';
	import Transition from '$components/Transition.svelte';
	import Navbar from '$components/navbar/Navbar.svelte';

	import { modpackJoins, setModpackJoins } from '$stores/modpackJoins';
	import { onVisible } from '$utils/onVisible';

	import type { LayoutData } from '../$types';
	export let data: LayoutData;

	onMount(() => {
		let window = WebviewWindow.getByLabel('main');

		if (window) onVisible(window, getModpackJoins);
	});

	async function getModpackJoins() {
		const res = await invoke('get_all_modpack_joins');

		// Check if res is an array of modpackjoins
		if (Array.isArray(res)) {
			setModpackJoins(res);
		}
	}
</script>

<Titlebar />
<div class="flex mt-8 w-screen bg-zinc-800 text-slate-50 select-none">
	<Navbar modpacks={$modpackJoins} />
	<Transition url={data.url}>
		<slot />
	</Transition>
</div>

<style>
	div {
		height: calc(100vh - 2rem);
	}
</style>
