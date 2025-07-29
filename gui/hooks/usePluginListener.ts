import { Message } from '@/bindings/Message';

import { useEffect } from 'react';

export function usePluginListener(callback: (message: Message) => void) {
	useEffect(() => {
		const unsubscribe = window.plugin.listen((message) =>
			callback(JSON.parse(message))
		);
		return () => {
			unsubscribe();
		};
	}, [callback]);
}
