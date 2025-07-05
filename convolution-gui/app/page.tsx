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
				<div className='flex-col h-[100vh] w-[100vw] px-1'>
					<div className='flex justify-between gap-2 items-center h-[5vh] py-1 '>
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

					{/* ALL processing */}
					<div className='flex gap-1 py-1 h-[60vh]'>
						<div className='w-[60%] h-full flex bg-zinc-700 rounded-sm p-1 gap-1'>
							{/* LP column */}
							<div className='w-[33%] h-full flex flex-col items-center justify-between bg-zinc-500 rounded-sm'>
								<h1>Lowpass</h1>
								<button>enabled?</button>
								<Knob
									parameter='lowpass_freq'
									label={'Freq.'}
									size={36}
									//
									cosmeticDefaultValue={dbToGain(0)}
									cosmeticRange={
										new NormalisableRange(
											dbToGain(-30),
											dbToGain(30),
											dbToGain(0)
										)
									}
									valueRawDisplayFn={(x) => x.toFixed(2)}
								/>
								<Knob
									parameter='lowpass_q'
									label={'Q'}
									size={36}
									//
									cosmeticDefaultValue={dbToGain(0)}
									cosmeticRange={
										new NormalisableRange(
											dbToGain(-30),
											dbToGain(30),
											dbToGain(0)
										)
									}
									valueRawDisplayFn={(x) => x.toFixed(2)}
								/>
							</div>
							{/* BELL COLUMN */}
							<div className='w-[33%] h-full flex flex-col items-center justify-between bg-zinc-500 rounded-sm'>
								<h1>Bell</h1>
								<button>enabled?</button>
								<Knob
									parameter='bell_freq'
									label={'Freq.'}
									size={36}
									//
									cosmeticDefaultValue={dbToGain(0)}
									cosmeticRange={
										new NormalisableRange(
											dbToGain(-30),
											dbToGain(30),
											dbToGain(0)
										)
									}
									valueRawDisplayFn={(x) => x.toFixed(2)}
								/>
								<Knob
									parameter='bell_q'
									label={'Q'}
									size={36}
									//
									cosmeticDefaultValue={dbToGain(0)}
									cosmeticRange={
										new NormalisableRange(
											dbToGain(-30),
											dbToGain(30),
											dbToGain(0)
										)
									}
									valueRawDisplayFn={(x) => x.toFixed(2)}
								/>
								<Knob
									parameter='bell_gain'
									label={'Gain'}
									size={36}
									//
									cosmeticDefaultValue={dbToGain(0)}
									cosmeticRange={
										new NormalisableRange(
											dbToGain(-30),
											dbToGain(30),
											dbToGain(0)
										)
									}
									valueRawDisplayFn={(x) => x.toFixed(2)}
								/>
							</div>

							{/* HP Column */}
							<div className='w-[33%] h-full flex flex-col items-center justify-between bg-zinc-500 rounded-sm'>
								<h1>Highpass</h1>
								<button>enabled?</button>
								<Knob
									parameter='highpass_freq'
									label={'Freq.'}
									size={36}
									//
									cosmeticDefaultValue={dbToGain(0)}
									cosmeticRange={
										new NormalisableRange(
											dbToGain(-30),
											dbToGain(30),
											dbToGain(0)
										)
									}
									valueRawDisplayFn={(x) => x.toFixed(2)}
								/>
								<Knob
									parameter='highpass_q'
									label={'Q'}
									size={36}
									//
									cosmeticDefaultValue={dbToGain(0)}
									cosmeticRange={
										new NormalisableRange(
											dbToGain(-30),
											dbToGain(30),
											dbToGain(0)
										)
									}
									valueRawDisplayFn={(x) => x.toFixed(2)}
								/>
							</div>
						</div>
						{/* GAIN CONTROLS */}
						<div className='w-[40%] h-full'>
							<div className='w-full h-full flex flex-col items-center justify-center gap-5 bg-zinc-700 rounded-sm'>
								<div className='bg-zinc-500 rounded-sm'>
									<Knob
										parameter='highpass_q'
										label={'Dry Gain'}
										size={64}
										//
										cosmeticDefaultValue={dbToGain(0)}
										cosmeticRange={
											new NormalisableRange(
												dbToGain(-30),
												dbToGain(30),
												dbToGain(0)
											)
										}
										valueRawDisplayFn={(x) => x.toFixed(2)}
									/>
									<Knob
										parameter='highpass_q'
										label={'Wet Gain'}
										size={64}
										//
										cosmeticDefaultValue={dbToGain(0)}
										cosmeticRange={
											new NormalisableRange(
												dbToGain(-30),
												dbToGain(30),
												dbToGain(0)
											)
										}
										valueRawDisplayFn={(x) => x.toFixed(2)}
									/>
								</div>
							</div>
						</div>
					</div>
				</div>
			</GlobalParametersContext.Provider>
		</MessageBusContext.Provider>
	);
}
