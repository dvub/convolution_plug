import { Message } from '@/bindings/Message';
import { MessageBusContext } from '@/contexts/MessageBusContext';
import { sendToPlugin } from '@/lib';
import { GearIcon, UploadIcon, ValueNoneIcon } from '@radix-ui/react-icons';

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
			waveColor: 'white',
			normalize: true,
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
		<div className='rounded-sm w-full'>
			<div>
				<div ref={containerRef} className='w-full h-[30vh]' />
			</div>

			<div className='flex items-center justify-between gap-2 border-t-1 text-md h-[5vh]'>
				<input
					id='inp'
					type='file'
					onChange={onFileChange}
					className='hidden'
				/>
				<label
					htmlFor='inp'
					className='p-1 hover:cursor-pointer border-r-1 rounded-r-sm'
				>
					<UploadIcon />
				</label>
				<div className='w-full'>
					<h1>{fileName}</h1>
				</div>

				<button className='p-1 hover:cursor-pointer border-l-1 rounded-l-sm'>
					<ValueNoneIcon />
				</button>
				<button className='p-1 hover:cursor-pointer border-l-1 rounded-l-sm'>
					<GearIcon />
				</button>
			</div>
		</div>
	);
}
