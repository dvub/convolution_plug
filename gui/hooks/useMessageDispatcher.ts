import { MessageBus } from '@/contexts/MessageBusContext';
import { useEffect } from 'react';

export function useMessageDispatcher(messageBus: MessageBus) {
	useEffect(() => {
		window.plugin.onmessage = (message) => {
			messageBus.dispatch(JSON.parse(message));
		};
	}, [messageBus]);
}
