import { Message } from '@/bindings/Message';

export function sendToPlugin(msg: Message) {
	if (!window) {
		console.log('Window not available.');
		return;
	}
	window.plugin.send(JSON.stringify(msg));
}
