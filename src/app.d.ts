// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	type Mod = {
		id: number;
		project_id: string;
		name: string;
		slug: string;
		page_url: string;
	};

	type ModJoin = {
		id: number;
		mod_id: number;
		version_id: string;
		name: string;
		slug: string;
		game_version: string;
		page_url: string;
		download_url: string;
	};

	type Modpack = {
		id: number;
		name: string;
		slug: string;
		premade: boolean;
	};

	type ModpackJoin = {
		id: number;
		modpack_id: number;
		name: string;
		slug: string;
		game_version: string;
		premade: boolean;
		installed: boolean;
		loaded: boolean;
	};

	type NewMod = {
		id: null;
		project_id: string;
		name: string;
		slug: string;
		page_url: string;
	};

	type NewModpack = {
		id: null;
		name: string;
		slug: string;
		premade: boolean;
	};

	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface Platform {}
	}
}

export {};
