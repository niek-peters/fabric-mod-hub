<script>
	import Fa from 'svelte-fa';
	import { faMinus, faUpRightFromSquare } from '@fortawesome/free-solid-svg-icons';
</script>

<div class="flex flex-col gap-8 mt-4">
	<section class="relative flex items-center justify-center">
		<h1 class="absolute text-5xl text-center z-10">Welcome to Fabric Mod Hub</h1>
		<img class="w-64 brightness-50" src="icon.svg" alt="Fabric Mod Hub icon" />
	</section>
	<article class="flex flex-col gap-4 w-full">
		<h2 class="text-xl">Currently loaded modpack</h2>
		<div class="flex gap-4">
			<section class="flex flex-col gap-4 bg-zinc-700/20 p-4 rounded-md flex-grow shadow-2xl">
				<div class="flex justify-between">
					<h3 class="text-2xl">Optimizations</h3>
					<span class="font-semibold">1.19.2</span>
				</div>
				<section class="flex items-center">
					<span class="bg-creeper/80 px-4 py-px rounded-full">Pre-made</span>
				</section>
			</section>
			<section class="flex flex-col gap-4 w-1/3">
				<button class="flex items-center gap-4 px-4 flex-grow bg-indigo-800 rounded-md shadow-2xl"
					><Fa class="text-lg" icon={faUpRightFromSquare} />
					<p>View</p></button
				>
				<button class="flex items-center gap-4 px-4 flex-grow bg-rose-900 rounded-md shadow-2xl"
					><Fa class="text-lg" icon={faMinus} />
					<p>Unload</p></button
				>
			</section>
		</div>
	</article>
</div>
