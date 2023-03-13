<script>
	import { appWindow } from '@tauri-apps/api/window';

	import Fa from 'svelte-fa';
	import { faMinus, faXmark } from '@fortawesome/free-solid-svg-icons';
</script>

<div
	data-tauri-drag-region
	class="fixed flex justify-between left-0 top-0 w-full h-8 px-3 bg-zinc-850 text-xl z-50 select-none"
>
	<section data-tauri-drag-region class="flex items-center gap-2 text-zinc-300 text-lg">
		Fabric Mod Hub
	</section>
	<section class="flex gap-4">
		<button on:click={() => appWindow.minimize()}>
			<Fa class="text-zinc-500 hover:text-zinc-400 transition duration-300" icon={faMinus} />
		</button>
		<button on:click={() => appWindow.close()}>
			<Fa class="text-zinc-500 hover:text-zinc-400 transition duration-300" icon={faXmark} />
		</button>
	</section>
</div>

<style>
	div {
		border-radius: var(--window-border-radius) var(--window-border-radius) 0 0;
		overflow: hidden;
	}

	.bg-zinc-850 {
		background-color: rgb(31, 31, 35);
	}

	.select-none {
		user-select: none;
	}
</style>
