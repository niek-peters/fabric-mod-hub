<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';

	import toast, { Toaster } from 'svelte-french-toast';

	import Titlebar from '$components/Titlebar.svelte';
	import Transition from '$components/Transition.svelte';
	import Navbar from '$components/navbar/Navbar.svelte';

	import { modpackJoins, setModpackJoins } from '$stores/modpackJoins';
	import { onVisible } from '$utils/onVisible';

	import type { LayoutData } from '../$types';
	import { setFoundMcDir } from '$src/lib/stores/foundMcDir';
	import { goto } from '$app/navigation';
	export let data: LayoutData;

	onMount(async () => {
		let window = WebviewWindow.getByLabel('main');

		if (window) {
			try {
				await invoke('init_settings');
			} catch (_e) {
				toast('Please pick your Minecraft folder', {
					icon: '👇',
					style: 'background-color: #52525b; color: #e4e4e7; border-radius: 0.375rem;'
				});
				setFoundMcDir(false);
				goto('/settings');
			}

			onVisible(window, getModpackJoins);
		}
	});

	async function getModpackJoins() {
		const res = await invoke('get_all_modpack_joins');

		// Check if res is an array of modpackjoins
		if (Array.isArray(res)) {
			setModpackJoins(res);
		}
	}
</script>

<Toaster containerStyle="margin-top: 2rem; z-index: 40;" />

<Titlebar />
<div class="window flex relative mt-8 w-screen bg-zinc-800 text-slate-50 select-none">
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
