// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	type Mod = {
		id: number | null;
		project_id: string;
		name: string;
		slug: string;
		page_url: string;
	};

	type ModJoin = {
		id: number | null;
		mod_id: number;
		version_id: string;
		name: string;
		slug: string;
		game_version: string;
		page_url: string;
		download_url: string;
	};

	type Modpack = {
		id: number | null;
		name: string;
		slug: string;
		premade: boolean;
	};

	type ModpackJoin = {
		id: number | null;
		modpack_id: number;
		name: string;
		slug: string;
		game_version: string;
		premade: boolean;
		installed: boolean;
		loaded: boolean;
	};

	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface Platform {}
	}
}

export {};
