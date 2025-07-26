import { Message } from '@/bindings/Message';

export function sendToPlugin(msg: Message) {
	if (!window) {
		console.log('Window not available.');
		return;
	}
	window.plugin.postMessage(JSON.stringify(msg));
}

export function initializePlugin() {
	window.plugin = window.__NIH_PLUG_WEBVIEW__;
}
