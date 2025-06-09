import { MessageBus } from '@/contexts/MessageBusContext';
import { useEffect } from 'react';

export function useEventDispatcher(messageBus: MessageBus) {
	useEffect(() => {
		window.onPluginMessage = (message) => {
			messageBus.dispatch(message);
		};
	}, []);
}
