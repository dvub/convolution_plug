'use client';

import { MessageBus, MessageBusContext } from '@/contexts/MessageBusContext';
import { useEventDispatcher } from '@/hooks/useEventDispatcher';
import { useEffect, useState } from 'react';
import { PluginParams } from '../../bindings/PluginParams';
import { GlobalParametersContext } from '@/contexts/GlobalParamsContext';

export default function Home() {
	console.log('Sanity check.');

	const [messageBus] = useState(new MessageBus());
	const [parameters, setParameters] = useState<PluginParams>({
		gain: 0,
		dry_wet: 0,
		lowpass_enabled: false,
		lowpass_freq: 0,
		lowpass_q: 0,
		bell_enabled: false,
		bell_freq: 0,
		bell_q: 0,
		bell_gain: 0,
		highpass_enabled: false,
		highpass_freq: 0,
		highpass_q: 0,
	});

	useEventDispatcher(messageBus);

	useEffect(() => {
		const handlePluginMessage = (event: unknown) => {
			console.log(event);
		};

		const unsubscribe = messageBus.subscribe(handlePluginMessage);

		return () => {
			unsubscribe();
		};
	}, [parameters, messageBus]);

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
