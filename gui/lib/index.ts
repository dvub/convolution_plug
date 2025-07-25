import { IPC } from '@/app/thing';
import { Message } from '@/bindings/Message';

export function sendToPlugin(msg: Message) {
	IPC.send(JSON.stringify(msg));
}
