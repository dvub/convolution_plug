'use client';

import { MessageBus, MessageBusContext } from '@/contexts/MessageBusContext';
import { useEventDispatcher } from '@/hooks/useEventDispatcher';
import { useEffect, useState } from 'react';
import {
	GlobalParameters,
	GlobalParametersContext,
} from '@/contexts/GlobalParamsContext';

import { Message } from '@/bindings/Message';

import { sendToPlugin } from '@/lib';
import { Knob } from '@/components/knobs/Knob';
import { dbToGain, gainToDb, NormalisableRange } from '@/lib/utils';

import { FileInput } from '@/components/fileInput';

export default function Home() {
	const [messageBus] = useState(new MessageBus());
	const [parameters, setParameters] = useState<GlobalParameters>({
		gain: 0,
		dry_wet: 0,

		lowpass_enabled: 0,
		lowpass_freq: 0,
		lowpass_q: 0,

		bell_enabled: 0,
		bell_freq: 0,
		bell_q: 0,
		bell_gain: 0,

		highpass_enabled: 0,
		highpass_freq: 0,
		highpass_q: 0,
	});

	useEventDispatcher(messageBus);

	useEffect(() => {
		sendToPlugin({ type: 'init' });

		const handlePluginMessage = (event: Message) => {
			console.log(event);
			switch (event.type) {
				case 'parameterUpdate':
					setParameters((prevState) => {
						return {
							...prevState,
							[event.data.parameterId]: event.data.value,
						};
					});

					break;
			}
		};

		const unsubscribe = messageBus.subscribe(handlePluginMessage);

		return () => {
			unsubscribe();
		};
	}, []);

	return (
		<MessageBusContext.Provider value={messageBus}>
			<GlobalParametersContext.Provider
				value={{ parameters, setParameters }}
			>
				<FileInput />
				<Knob
					parameter='gain'
					label={'Gain'}
					size={50}
					//
					cosmeticDefaultValue={dbToGain(0)}
					cosmeticRange={
						new NormalisableRange(
							dbToGain(-30),
							dbToGain(30),
							dbToGain(0)
						)
					}
					valueRawDisplayFn={(x) => {
						let g = gainToDb(x).toFixed(2);
						// TODO: make this not be really scuffed
						if (g === '-0.00') {
							g = '0.00';
						}
						return `${g} dB`;
					}}
				></Knob>
			</GlobalParametersContext.Provider>
		</MessageBusContext.Provider>
	);
}
