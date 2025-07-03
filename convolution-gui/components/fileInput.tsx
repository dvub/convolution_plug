import { sendToPlugin } from '@/lib';
import { useWavesurfer } from '@wavesurfer/react';
import { ChangeEvent, useRef, useState } from 'react';

export function FileInput(props: { peaks: number[] | null }) {
	console.log('this component rerednered');
	console.log('HI:', props.peaks);
	const [fileName, setFileName] = useState('...');

	const containerRef = useRef(null);
	const { wavesurfer } = useWavesurfer({
		url: '',
		container: containerRef,
		waveColor: 'purple',
		normalize: true,
		height: 200,
	});

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
		wavesurfer?.loadBlob(input);

		if (props.peaks) {
			wavesurfer?.load('', [props.peaks]);
		}

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
