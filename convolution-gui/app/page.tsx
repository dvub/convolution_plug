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

import { IrLoader } from '@/components/IRLoader';
import { GearIcon } from '@radix-ui/react-icons';

export default function Home() {
	const [isLoading, setIsLoading] = useState(true);

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
			if (isLoading) {
				setIsLoading(false);
			}

			if (event.type === 'parameterUpdate') {
				setParameters((prevState) => {
					return {
						...prevState,
						[event.data.parameterId]: event.data.value,
					};
				});
			}
		};
		const unsubscribe = messageBus.subscribe(handlePluginMessage);
		return () => {
			unsubscribe();
		};
	}, [messageBus]);

	return (
		<MessageBusContext.Provider value={messageBus}>
			<GlobalParametersContext.Provider
				value={{ parameters, setParameters }}
			>
				<div className='flex-col h-[100vh] w-[100vw]'>
					<div className='flex justify-between gap-2 items-center h-[5vh]'>
						<h1>CONVOLUTION</h1>
						<div className='w-full text-right'>
							<h1>dvub</h1>
						</div>

						<GearIcon />
					</div>
					<IrLoader />

					{/* <Knob
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
					/>*/}
					<div className='w-[60%] bg-red-500 h-[60vh] '></div>
				</div>
			</GlobalParametersContext.Provider>
		</MessageBusContext.Provider>
	);
}
