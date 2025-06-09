declare global {
	interface Window {
		ipc: { postMessage: (message: string) => void };

		onPluginMessage: ((message: unknown) => void) | null;
	}
}
