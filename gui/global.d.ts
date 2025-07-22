import { Message } from "./bindings/Message";

declare global {
	interface Window {
		ipc: { postMessage: (message: string) => void };

		onPluginMessage: ((message: Message) => void) | null;
	}
}
