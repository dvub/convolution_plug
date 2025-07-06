import { Message } from '@/bindings/Message';
import { MessageBusContext } from '@/contexts/MessageBusContext';
import { sendToPlugin } from '@/lib';
import { GearIcon, UploadIcon } from '@radix-ui/react-icons';

import { ChangeEvent, useContext, useEffect, useRef, useState } from 'react';
import WaveSurfer from 'wavesurfer.js';

export function IrLoader() {
	const waveSurferRef = useRef<WaveSurfer | null>(null);
	const containerRef = useRef(null);

	const messageBus = useContext(MessageBusContext)!;
	const [fileName, setFileName] = useState('...');

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
				setFileName(event.data.name);
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

		// first, configure the reader & behavior
		const reader = new FileReader();
		reader.onload = () => {
			const arrayBuffer = reader.result as ArrayBuffer;
			const bytes = new Uint8Array(arrayBuffer);

			sendToPlugin({
				type: 'irUpdate',
				data: {
					name: fileName,
					rawBytes: [...bytes],
				},
			});
		};
		// now pass the file into the reader

		// we can also be sure that there will be a file here (i think)
		const input = event.target.files[0];
		reader.readAsArrayBuffer(input);

		// finally, visualization
		waveSurferRef.current?.loadBlob(input);

		setFileName(fileName);
	}
	return (
		<div className='w-full secondary rounded-sm h-[35vh]'>
			<div className='h-[30vh] p-1'>
				<div className='h-full rounded-sm'>
					<div ref={containerRef} className='h-full' />
				</div>
			</div>

			<div className='h-[5vh] px-1 pb-1'>
				<div className='h-full flex items-center justify-between gap-2 text-sm rounded-sm accent'>
					<input
						id='inp'
						type='file'
						onChange={onFileChange}
						className='hidden'
					/>
					<label htmlFor='inp' className='p-1 hover:cursor-pointer'>
						<UploadIcon />
					</label>
					<div className='w-full'>
						<h1>{fileName}</h1>
					</div>

					<button className='p-1 hover:cursor-pointer'>
						<GearIcon />
					</button>
				</div>
			</div>
		</div>
	);
}
