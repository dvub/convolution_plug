import { MessageBus } from '@/contexts/MessageBusContext';
import { useEffect } from 'react';

export function useMessageDispatcher(messageBus: MessageBus) {
	useEffect(() => {
		window.onPluginMessage = (message) => {
			messageBus.dispatch(message);
		};
	}, [messageBus]);
}
