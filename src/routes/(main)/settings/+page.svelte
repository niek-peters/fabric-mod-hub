<script lang="ts">
	import { open } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	import Fa from 'svelte-fa';
	import { faFolder } from '@fortawesome/free-solid-svg-icons';
	import toast from 'svelte-french-toast';

	import Line from '$components/Line.svelte';

	import { foundMcDir, setFoundMcDir } from '$stores/foundMcDir';
	import { unload } from '$stores/modpackJoins';

	import type { PageData } from './$types';
	export let data: PageData;

	let allowUnstableEl: HTMLInputElement;
	let allowSnapshotsEl: HTMLInputElement;

	onMount(() => {
		allowUnstableEl.checked = data.settings.allow_unstable;
		allowSnapshotsEl.checked = data.settings.allow_snapshots;
	});

	async function pickFolder() {
		const selected = await open({
			directory: true
		});
		if (selected && typeof selected === 'string') {
			toast.promise(
				setMinecraftDir(selected),
				{
					loading: 'Changing Minecraft folder...',
					error: 'Failed to change Minecraft folder',
					success: 'Changed Minecraft folder'
				},
				{
					style: 'background-color: #52525b; color: #e4e4e7; border-radius: 0.375rem;'
				}
			);
		}
	}

	async function setMinecraftDir(dir: string) {
		unload();

		await invoke('set_minecraft_dir', {
			minecraftDir: dir
		});
		data.settings.minecraft_dir = dir;
		setFoundMcDir(true);
	}
</script>

<div class="flex flex-col gap-8">
	<section class="flex flex-col gap-4">
		<h2 class="text-2xl">Minecraft folder location</h2>
		<div class="flex gap-4 items-center text-lg">
			<button
				class="flex gap-2 items-center bg-zinc-700/50 hover:bg-zinc-700/80 px-4 py-2 rounded-md transition duration-300"
				on:click={pickFolder}
				><Fa icon={faFolder} class="text-2xl" />
				<p>Pick folder</p></button
			>
			<p class="folder-name whitespace-nowrap overflow-x-auto text-zinc-300 pointer-events-auto">
				Current folder: <span class="text-white">{data.settings.minecraft_dir}</span>
			</p>
		</div>
	</section>
	<Line />
	<section
		class="flex flex-col gap-4 mc-dir {!$foundMcDir
			? 'no-mc-dir pointer-events-none'
			: 'pointer-events-auto'}"
	>
		<h2 class="text-2xl">Installing modpacks</h2>
		<div class="flex gap-4 items-center text-lg">
			<input
				type="checkbox"
				id="allow_unstable"
				class="bg-zinc-700/50 hover:bg-zinc-700/80 h-8 w-8 rounded-md transition duration-300"
				bind:this={allowUnstableEl}
				on:change={async () => {
					data.settings.allow_unstable = allowUnstableEl.checked;
					await invoke('set_allow_unstable', { allowUnstable: data.settings.allow_unstable });
				}}
			/>
			<label for="allow_unstable">Allow unstable mod versions</label>
		</div>
		<div class="flex gap-4 items-center text-lg">
			<input
				type="checkbox"
				id="allow_snapshots"
				class="bg-zinc-700/50 hover:bg-zinc-700/80 h-8 w-8 rounded-md transition duration-300"
				bind:this={allowSnapshotsEl}
				on:change={async () => {
					data.settings.allow_snapshots = allowSnapshotsEl.checked;
					await invoke('set_allow_snapshots', { allowSnapshots: data.settings.allow_snapshots });
				}}
			/>
			<label for="allow_snapshots">Show snapshot versions</label>
		</div>
	</section>
</div>

<style lang="scss">
	.mc-dir {
		filter: blur(0px);
		transition: filter 300ms cubic-bezier(0.4, 0, 0.2, 1);

		&.no-mc-dir {
			filter: blur(2px);
		}
	}

	.folder-name {
		width: 37.35rem;

		/* width */
		&::-webkit-scrollbar {
			height: 0.5rem;
		}

		/* Track */
		&::-webkit-scrollbar-track {
			background-color: rgb(63 63 70 / 0.5);
			border-radius: 0.375rem /* 6px */;
		}

		/* Handle */
		&::-webkit-scrollbar-thumb {
			background-color: rgb(82 82 91 / 0.7);
			transition: all 300ms cubic-bezier(0.4, 0, 0.2, 1);
			border-radius: 0.375rem /* 6px */;

			&:hover {
				background-color: rgb(82 82 91 / 1);
			}
		}
	}
</style>
