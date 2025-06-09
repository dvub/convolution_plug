import { createContext } from 'react';

type Callback = (message: unknown) => void;

export class MessageBus {
	private listeners: Set<Callback> = new Set();

	subscribe(callback: Callback) {
		this.listeners.add(callback);
		console.log(
			'subscription added to message bus, current size:',
			this.listeners.size
		);

		return () => this.listeners.delete(callback);
	}

	dispatch(message: unknown) {
		this.listeners.forEach((callback) => callback(message));
	}
}

export const MessageBusContext = createContext<MessageBus | null>(null);
