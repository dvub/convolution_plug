'use client';

import { MessageBus, MessageBusContext } from '@/contexts/MessageBusContext';
import { useEventDispatcher } from '@/hooks/useEventDispatcher';
import { useEffect, useState } from 'react';

import { GlobalParametersContext } from '@/contexts/GlobalParamsContext';

import { Message } from '@/bindings/Message';

import { sendToPlugin } from '@/lib';

export default function Home() {
	const [messageBus] = useState(new MessageBus());
	const [parameters, setParameters] = useState({
		gain: 0,
		dryWet: 0,
	});

	useEventDispatcher(messageBus);

	useEffect(() => {
		sendToPlugin({ type: 'windowOpened' });

		const handlePluginMessage = (event: Message) => {
			console.log(event);
			switch (event.type) {
				case 'parameterUpdate':
					break;
			}
		};

		const unsubscribe = messageBus.subscribe(handlePluginMessage);

		return () => {
			sendToPlugin({ type: 'windowClosed' });

			unsubscribe();
		};
	}, []);

	return (
		<MessageBusContext.Provider value={messageBus}>
			<GlobalParametersContext.Provider
				value={{ parameters, setParameters }}
			>
				<h1>hello world</h1>
				<button
					onClick={() =>
						sendToPlugin({
							type: 'parameterUpdate',
							data: { parameter: 'highpassEnabled', value: true },
						})
					}
				>
					BUTTON
				</button>
			</GlobalParametersContext.Provider>
		</MessageBusContext.Provider>
	);
}
