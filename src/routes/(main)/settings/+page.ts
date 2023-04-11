import type { PageLoad } from './$types';

import { invoke } from '@tauri-apps/api/tauri';

export const load: PageLoad = async () => {
	const settings = (await invoke('get_settings')) as Settings;

	return {
		settings
	};
};
