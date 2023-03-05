<script>
	import Fa from 'svelte-fa';
	import {
		faCaretRight,
		faHome,
		faMagnifyingGlass,
		faPlus
	} from '@fortawesome/free-solid-svg-icons';
</script>

<nav class="flex flex-col w-64 h-screen bg-zinc-700/30 p-4">
	<section class="flex flex-col gap-6">
		<a href="/" class="flex items-center gap-4 py-2 px-4 rounded-md bg-zinc-700">
			<Fa icon={faHome} />
			<p>Home</p>
		</a>
		<span class="h-px w-full bg-zinc-700" />
		<div class="flex flex-col gap-4">
			<div class="flex bg-zinc-700/30 py-2 px-4 gap-4 rounded-full items-center">
				<Fa icon={faMagnifyingGlass} />
				<input
					class="bg-transparent placeholder-slate-200"
					type="text"
					placeholder="Search modpacks"
				/>
			</div>
			<section class="flex flex-col gap-4">
				<a
					href="/"
					class="relative flex items-center justify-between gap-4 py-2 px-4 rounded-md bg-zinc-700/50"
				>
					<p>Optimizations</p>
					<p class="text-sm text-slate-200">1.19.2</p>

					<Fa class="absolute text-3xl text-creeper -left-2 shadow-2xl" icon={faCaretRight} />
				</a><a
					href="/"
					class="relative flex items-center justify-between gap-4 py-2 px-4 rounded-md bg-zinc-700/50"
				>
					<p>Multiplayer</p>
					<p class="text-sm text-slate-200">1.19.2</p>

					<Fa class="absolute text-3xl text-transparent -left-2 shadow-2xl" icon={faCaretRight} />
				</a><a
					href="/"
					class="relative flex items-center justify-between gap-4 py-2 px-4 rounded-md bg-zinc-700/50"
				>
					<p>Hardcore</p>
					<p class="text-sm text-slate-200">1.18.2</p>

					<Fa class="absolute text-3xl text-transparent -left-2 shadow-2xl" icon={faCaretRight} />
				</a><a
					href="/"
					class="relative flex items-center justify-between gap-4 py-2 px-4 rounded-md bg-zinc-700/50"
				>
					<p>QoL</p>
					<p class="text-sm text-slate-200">1.18.2</p>

					<Fa class="absolute text-3xl text-transparent -left-2 shadow-2xl" icon={faCaretRight} />
				</a>
			</section>
		</div>
		<span class="h-px w-full bg-zinc-700" />
		<a href="/" class="flex items-center gap-4 py-2 px-4 rounded-md bg-zinc-700/50">
			<Fa icon={faPlus} />
			<p>New modpack</p>
		</a>
	</section>
	<section class="mt-auto flex">
		<p>&copy; Niek Peters 2023</p>
	</section>
</nav>
