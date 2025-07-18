import { IrData } from '@/bindings/IrData';
import { Message } from '@/bindings/Message';

import { useMessageSubscriber } from '@/hooks/useMessageSubscriber';

import { useWaveform } from '@/hooks/useWaveform';
import { IRInput } from './IRInput';
import { useRef, useState } from 'react';
import { NormalizeControls } from './NormalizeControls';
import { ResampleControls } from './ResampleControls';
import { IrConfig } from '@/bindings/IrConfig';
export function IRManager() {
	const containerRef = useRef(null);
	const waveSurferRef = useWaveform(containerRef);

	const [irData, setIrData] = useState<IrData | undefined>();
	const [irConfig, setIrConfig] = useState<IrConfig | undefined>();
	useMessageSubscriber((message: Message) => {
		if (message.type !== 'initResponse') {
			return;
		}
		if (message.data.irData) {
			setIrData(message.data.irData);
		}
	});

	// TODO: probably want to refactor this
	const IrInfoDisplay = (
		<>
			<h1 className='text-sm'>
				{/* https://stackoverflow.com/questions/1199352/smart-way-to-truncate-long-strings*/}
				{irData?.name.replace(/(.{20})..+/, '$1…')}
			</h1>
			<p className='text-xs'>
				Length: {irData?.lengthSeconds.toFixed(3)}s
				<br />
				{irData?.numChannels} Channels
				<br />
				{irData?.sampleRate} Hz
				<br />
			</p>
		</>
	);
	const defaultIrDisplay = <h1>No IR Loaded.</h1>;

	return (
		<div className='w-full h-[35vh] flex gap-1'>
			<div
				ref={containerRef}
				className='w-[50%] h-full rounded-sm secondary'
			/>

			<div className='w-[50%] flex flex-col gap-1'>
				<div className='secondary rounded-sm p-1'>
					{irData ? IrInfoDisplay : defaultIrDisplay}
				</div>
				<div className='h-full secondary rounded-sm p-1 text-xs flex flex-col justify-between text-center'>
					<div className='flex flex-col gap-1'>
						<NormalizeControls
							irConfig={irConfig!}
							setIrConfig={setIrConfig}
						/>
						<ResampleControls
							irConfig={irConfig!}
							setIrConfig={setIrConfig}
						/>
					</div>
					<IRInput
						waveSurferRef={waveSurferRef}
						setFileInfo={setIrData}
					/>
				</div>
			</div>
		</div>
	);
}
