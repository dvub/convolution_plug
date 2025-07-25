import { IPC } from '@/app/thing';
import { MessageBus } from '@/contexts/MessageBusContext';
import { useEffect } from 'react';

export function useMessageDispatcher(messageBus: MessageBus) {
	useEffect(() => {
		// TODO: change this
		IPC.on((message) => {
			console.log('FROM DISPATCH:', message);
			messageBus.dispatch(JSON.parse(message));
		});
	}, [messageBus]);
}
