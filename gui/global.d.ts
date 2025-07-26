interface Plugin {
	onmessage: (message: string) => void;
	postMessage: (message: string) => void;
}

declare global {
	interface Window {
		__NIH_PLUG_WEBVIEW__: Plugin;
		plugin: Plugin;
	}
}

export {};
