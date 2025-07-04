import { Message } from '@/bindings/Message';
import { MessageBusContext } from '@/contexts/MessageBusContext';
import { sendToPlugin } from '@/lib';

import { ChangeEvent, useContext, useEffect, useRef, useState } from 'react';
import WaveSurfer from 'wavesurfer.js';

export function FileInput() {
	const waveSurferRef = useRef<WaveSurfer | null>(null);
	const containerRef = useRef(null);

	const messageBus = useContext(MessageBusContext)!;
	const [fileName, setFileName] = useState('...');

	useEffect(() => {
		const waveSurfer = WaveSurfer.create({
			container: containerRef.current!,
			height: 200,
			waveColor: 'purple',
			normalize: true,
		});

		waveSurfer.on('init', () => {
			waveSurferRef.current = waveSurfer;
		});

		const handlePluginMessage = (event: Message) => {
			if (event.type === 'slotUpdate') {
				const blob = new Blob([new Uint8Array(event.data)], {
					type: 'wav',
				});
				waveSurfer.loadBlob(blob);
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

		// first, configure the reader & behavior
		const reader = new FileReader();
		reader.onload = () => {
			const arrayBuffer = reader.result as ArrayBuffer;
			const bytes = new Uint8Array(arrayBuffer);

			sendToPlugin({ type: 'slotUpdate', data: [...bytes] });
		};
		// now pass the file into the reader

		// we can also be sure that there will be a file here (i think)
		const input = event.target.files[0];
		reader.readAsArrayBuffer(input);

		// finally, visualization
		waveSurferRef.current?.loadBlob(input);

		setFileName(event.target.files[0].name);
	}
	return (
		<div className='m-1'>
			<div className='flex justify-between items-center my-1'>
				<h1>{fileName}</h1>
				<input
					id='inp'
					type='file'
					onChange={onFileChange}
					className='hidden'
				/>
				<label
					htmlFor='inp'
					className='bg-gray-500 border-2 rounded-sm p-1 hover:cursor-pointer'
				>
					Load IR
				</label>
			</div>

			<div ref={containerRef} className='border-2 rounded-sm w-full' />
		</div>
	);
}
