'use client';

import { MessageBus, MessageBusContext } from '@/contexts/MessageBusContext';
import { useEventDispatcher } from '@/hooks/useEventDispatcher';
import { useEffect, useState } from 'react';

import { GlobalParametersContext } from '@/contexts/GlobalParamsContext';
import { GUIParams } from '@/bindings/GUIParams';
import { Message } from '@/bindings/Message';

import { sendToPlugin } from '@/lib';

export default function Home() {
	const [messageBus] = useState(new MessageBus());
	const [parameters, setParameters] = useState<GUIParams>({
		gain: 0,
		dryWet: 0,
		lowpassEnabled: false,
		lowpassFreq: 0,
		lowpassQ: 0,
		/*bell_enabled: false,
		bell_freq: 0,
		bell_q: 0,
		bell_gain: 0,
		highpass_enabled: false,
		highpass_freq: 0,
		highpass_q: 0,*/
	});

	useEventDispatcher(messageBus);

	useEffect(() => {
		sendToPlugin({ type: 'windowOpened' });

		const handlePluginMessage = (event: Message) => {
			switch (event.type) {
				case 'parameterUpdate':
					console.log(event.data);
					setParameters(event.data);

					break;
			}

			console.log(event);
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
			</GlobalParametersContext.Provider>
		</MessageBusContext.Provider>
	);
}
