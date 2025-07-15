import { IrData } from '@/bindings/IrData';
import { Message } from '@/bindings/Message';

import { useMessageSubscriber } from '@/hooks/useMessageSubscriber';

import { useWaveform } from '@/hooks/useWaveform';
import { IRInput } from './IRInput';
import { useRef, useState } from 'react';
import { NormalizeControls } from './NormalizeControls';
import { ResampleControls } from './ResampleControls';

export function IRManager() {
	const containerRef = useRef(null);
	const waveSurferRef = useWaveform(containerRef);

	const [fileInfo, setFileInfo] = useState<IrData | undefined>(undefined);
	useMessageSubscriber((message: Message) => {
		if (message.type === 'initResponse' && message.data.irData) {
			setFileInfo(message.data.irData);
		}
	});

	// TODO: probably want to refactor this
	const IrInfoDisplay = (
		<>
			<h1 className='text-sm'>
				{/* https://stackoverflow.com/questions/1199352/smart-way-to-truncate-long-strings*/}
				{fileInfo?.name.replace(/(.{20})..+/, '$1…')}
			</h1>
			<p className='text-xs'>
				Length: {fileInfo?.lengthSeconds.toFixed(3)}s
				<br />
				{fileInfo?.numChannels} Channels
				<br />
				{fileInfo?.sampleRate} Hz
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
					{fileInfo ? IrInfoDisplay : defaultIrDisplay}
				</div>
				<div className='h-full secondary rounded-sm p-1 text-sm flex flex-col justify-between text-center'>
					<div className='flex flex-col gap-2'>
						<NormalizeControls />
						<ResampleControls />
					</div>

					<IRInput
						waveSurferRef={waveSurferRef}
						setFileInfo={setFileInfo}
					/>
				</div>
			</div>
		</div>
	);
}
