import { Message } from '@/bindings/Message';

export function sendToPlugin(msg: Message) {
	if (window.ipc) {
		window.ipc.postMessage(JSON.stringify(msg));
	}
}
