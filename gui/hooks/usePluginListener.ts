import { Message } from '@/bindings/Message';

import { useEffect } from 'react';

export function usePluginListener(callback: (message: Message) => void) {
	useEffect(() => {
		const unsubscribe = window.plugin.listen((m) =>
			callback(JSON.parse(m))
		);
		return () => {
			unsubscribe();
		};
	}, [callback]);
}
