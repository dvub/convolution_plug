import { Message } from '@/bindings/Message';
import { MessageBus, MessageBusContext } from '@/contexts/MessageBusContext';
import { useContext, useEffect } from 'react';

export function useMessageSubscriber(
	callback: (message: Message) => void,
	bus?: MessageBus
) {
	// TODO: ! might be an issue here
	const contextBus = useContext(MessageBusContext)!;
	let messageBus = bus;

	if (!messageBus) {
		messageBus = contextBus;
	}
	useEffect(() => {
		const unsubscribe = messageBus.subscribe(callback);
		return () => {
			unsubscribe();
		};
	}, [callback, messageBus]);
}
