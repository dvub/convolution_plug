import { IrData } from '@/bindings/IrData';
import { Message } from '@/bindings/Message';
import { MessageBusContext } from '@/contexts/MessageBusContext';
import { sendToPlugin } from '@/lib';
import { UploadIcon } from '@radix-ui/react-icons';
import { parseBuffer } from 'music-metadata';

import { ChangeEvent, useContext, useEffect, useRef, useState } from 'react';
import WaveSurfer from 'wavesurfer.js';

export function IrLoader() {
	const waveSurferRef = useRef<WaveSurfer | null>(null);
	const containerRef = useRef(null);

	const messageBus = useContext(MessageBusContext)!;
	const [fileInfo, setFileInfo] = useState<IrData | undefined>(undefined);

	useEffect(() => {
		const waveSurfer = WaveSurfer.create({
			container: containerRef.current!,
			height: 'auto',
			waveColor: '#ecf0ef',
			normalize: true,
			cursorWidth: 0,
			interact: false,
		});

		waveSurfer.on('init', () => {
			waveSurferRef.current = waveSurfer;
		});

		const handlePluginMessage = (event: Message) => {
			if (event.type === 'irUpdate') {
				const blob = new Blob([new Uint8Array(event.data.rawBytes)], {
					type: 'wav',
				});
				waveSurfer.loadBlob(blob);
				setFileInfo(event.data);
			}
		};

		const unsubscribe = messageBus.subscribe(handlePluginMessage);
		return () => {
			unsubscribe();
			waveSurfer.destroy();
		};
	}, [messageBus]);

	function onFileChange(event: ChangeEvent<HTMLInputElement>) {
		if (!event.target.files || event.target.files.length === 0) {
			return;
		}
		const fileName = event.target.files[0].name;

		// TODO: fix nested code
		const reader = new FileReader();
		reader.onload = () => {
			const arrayBuffer = reader.result as ArrayBuffer;
			const bytes = new Uint8Array(arrayBuffer);
			parseBuffer(bytes).then((x) => {
				const info = x.format;

				const irData: IrData = {
					name: fileName,
					rawBytes: [...bytes],
					// TODO: handle if these are undefined!
					lengthSeconds: info.duration!,
					numChannels: info.numberOfChannels!,
					sampleRate: info.sampleRate!,
				};

				setFileInfo(irData);

				sendToPlugin({
					type: 'irUpdate',
					data: irData,
				});
			});
		};
		// now pass the file into the reader

		// we can also be sure that there will be a file here (i think)
		const input = event.target.files[0];
		reader.readAsArrayBuffer(input);

		// finally, visualization
		waveSurferRef.current?.loadBlob(input);
	}
	return (
		<div className='w-full secondary rounded-sm h-[35vh]'>
			<div className='h-[35vh] p-1 flex gap-1'>
				<div className='w-[50%] h-full rounded-sm'>
					<div ref={containerRef} className='h-full' />
				</div>
				<div className='w-[50%] flex flex-col gap-1'>
					<div className='h-[50%] bg-zinc-500 rounded-sm p-1'>
						{fileInfo ? (
							<div>
								<h1 className='text-sm'>{fileInfo?.name}</h1>
								<h1 className='text-xs'>
									Len: {fileInfo?.lengthSeconds}s
								</h1>
								<h1 className='text-xs'>
									Channels: {fileInfo?.numChannels}
								</h1>
								<h1 className='text-xs'>
									SR: {fileInfo?.sampleRate} Hz
								</h1>
							</div>
						) : (
							<h1>...</h1>
						)}
					</div>
					<div className='h-[50%] bg-zinc-500 rounded-sm p-1'>
						<h1>Normalize | Off | ... </h1>
						<h1>Resample | Off | Current SR</h1>

						<div className='flex gap-5 items-center'>
							<input
								id='inp'
								type='file'
								onChange={onFileChange}
								className='hidden'
							/>
							<label
								htmlFor='inp'
								className='hover:cursor-pointer'
							>
								<UploadIcon />
							</label>
							<h1>Refresh</h1>
						</div>
					</div>
				</div>
			</div>
		</div>
	);
}
