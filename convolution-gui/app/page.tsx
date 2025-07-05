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
import { dbToGain, gainToDb } from '@/lib/conversion';

import { IrLoader } from '@/components/IRLoader';
import { GearIcon } from '@radix-ui/react-icons';
import {
	NumericRange,
	skewFactor,
	RangeType,
	gainSkewFactor,
} from '@/lib/range';
import { gainFormatter, hzThenKhz } from '@/lib/format';

const DEFAULT_FREQ_RANGE = new NumericRange(
	10,
	22050,
	skewFactor(-2.5),
	RangeType.Skewed
);
const DEFAULT_Q_RANGE = new NumericRange(
	0.1,
	18,
	skewFactor(-2),
	RangeType.Skewed
);

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

					{/* ALL processing */}
					<div className='flex gap-1 py-1 h-[60vh]'>
						{/* FILTERS */}
						<div className='w-[60%] h-full flex bg-zinc-700 rounded-sm p-1 gap-1 text-center'>
							{/* LP column */}
							<div className='w-[33%] h-full flex flex-col items-center justify-between bg-zinc-500 rounded-sm'>
								<div className='flex flex-col items-center gap-2'>
									<h1>Lowpass</h1>
									<div className='w-6 h-6 border-2 border-black flex justify-center items-center'>
										<div className='w-4 h-4 bg-black'></div>
									</div>
								</div>
								<Knob
									parameter='lowpass_freq'
									label='Freq.'
									size={36}
									defaultValue={10}
									range={DEFAULT_FREQ_RANGE}
									valueRawDisplayFn={(x) => hzThenKhz(x, 2)}
								/>
								<Knob
									parameter='lowpass_q'
									label='Q.'
									size={36}
									defaultValue={0.1}
									range={DEFAULT_Q_RANGE}
									valueRawDisplayFn={(x) => x.toFixed(2)}
								/>
							</div>
							{/* BELL COLUMN */}
							<div className='w-[33%] h-full flex flex-col items-center justify-between bg-zinc-500 rounded-sm'>
								<div className='flex flex-col items-center gap-2'>
									<h1>Bell</h1>
									<div className='w-6 h-6 border-2 border-black flex justify-center items-center'>
										<div className='w-4 h-4 bg-black'></div>
									</div>
								</div>

								<Knob
									parameter='bell_freq'
									label={'Freq.'}
									size={36}
									defaultValue={10}
									range={DEFAULT_FREQ_RANGE}
									valueRawDisplayFn={(x) => hzThenKhz(x, 2)}
								/>
								<Knob
									parameter='bell_q'
									label={'Q'}
									size={36}
									defaultValue={0.1}
									range={DEFAULT_Q_RANGE}
									valueRawDisplayFn={(x) => x.toFixed(2)}
								/>
								<Knob
									parameter='bell_gain'
									label={'Gain'}
									size={36}
									defaultValue={dbToGain(0)}
									range={
										new NumericRange(
											dbToGain(-15),
											dbToGain(15),
											gainSkewFactor(-15, 15),
											RangeType.Skewed
										)
									}
									valueRawDisplayFn={(x) =>
										gainFormatter(x, 2)
									}
								/>
							</div>

							{/* HP Column */}
							<div className='w-[33%] h-full flex flex-col items-center justify-between bg-zinc-500 rounded-sm'>
								<div className='flex flex-col items-center gap-2'>
									<h1>Highpass</h1>
									<div className='w-6 h-6 border-2 border-black flex justify-center items-center'>
										<div className='w-4 h-4 bg-black'></div>
									</div>
								</div>
								<Knob
									parameter='highpass_freq'
									label={'Freq.'}
									size={36}
									defaultValue={10}
									range={DEFAULT_FREQ_RANGE}
									valueRawDisplayFn={(x) => hzThenKhz(x, 2)}
								/>
								<Knob
									parameter='highpass_q'
									label={'Q'}
									size={36}
									defaultValue={0.1}
									range={DEFAULT_Q_RANGE}
									valueRawDisplayFn={(x) => x.toFixed(2)}
								/>
							</div>
						</div>
						{/* GAIN CONTROLS */}
						<div className='w-[40%] h-full'>
							<div className='w-full h-full flex flex-col items-center justify-center gap-5 bg-zinc-700 rounded-sm'>
								<div className='bg-zinc-500 rounded-sm p-10'>
									<Knob
										parameter='gain'
										label={'Gain'}
										size={50}
										defaultValue={dbToGain(0)}
										range={
											new NumericRange(
												dbToGain(-30),
												dbToGain(30),
												gainSkewFactor(-30, 30),
												RangeType.Skewed
											)
										}
										valueRawDisplayFn={(x) => {
											const g = gainToDb(x).toFixed(2);
											return `${g} dB`;
										}}
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
