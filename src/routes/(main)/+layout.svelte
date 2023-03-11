<script lang="ts">
	import Titlebar from '$components/Titlebar.svelte';
	import Transition from '$components/Transition.svelte';
	import Navbar from '$components/navbar/Navbar.svelte';
	import type { LayoutData } from '../$types';
	import { modpackJoins, setModpackJoins } from '$src/lib/stores/modpackJoins';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onVisible } from '$utils/onVisible';

	export let data: LayoutData;

	onMount(async () => {
		let window = WebviewWindow.getByLabel('main');

		if (window) onVisible(window, getModpackJoins);
	});

	async function getModpackJoins() {
		let res = await invoke('get_all_modpack_joins');

		// Check if res is an array of modpackjoins
		if (Array.isArray(res)) {
			setModpackJoins(res);
		}
	}
</script>

<Titlebar />
<div class="flex mt-8 w-screen bg-zinc-800 text-slate-50">
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
