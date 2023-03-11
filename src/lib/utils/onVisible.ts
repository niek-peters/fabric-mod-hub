import type { WebviewWindow } from '@tauri-apps/api/window';

export async function onVisible(window: WebviewWindow, cb: () => void) {
	while (!(await window.isVisible())) {
		await new Promise((resolve) => setTimeout(resolve, 50));
	}

	cb();
}
