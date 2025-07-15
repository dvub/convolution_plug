import { IrData } from '@/bindings/IrData';
import { sendToPlugin } from '@/lib';
import { IAudioMetadata, parseBuffer } from 'music-metadata';
import { ChangeEvent, Dispatch, RefObject, SetStateAction } from 'react';
import WaveSurfer from 'wavesurfer.js';

export function IRInput(props: {
	waveSurferRef: RefObject<WaveSurfer | null>;
	setFileInfo: Dispatch<SetStateAction<IrData | undefined>>;
}) {
	const { waveSurferRef, setFileInfo } = props;

	function onFileChange(event: ChangeEvent<HTMLInputElement>) {
		if (!event.target.files || event.target.files.length === 0) {
			return;
		}
		const fileName = event.target.files[0].name;
		const targetFile = event.target.files[0];

		const reader = new FileReader();
		reader.onload = () => {
			const arrayBuffer = reader.result as ArrayBuffer;
			const bytes = new Uint8Array(arrayBuffer);
			parseBuffer(bytes).then((metadata) =>
				handleIRLoaded(fileName, bytes, metadata)
			);
		};
		// we can also be sure that there will be a file here (i think)
		reader.readAsArrayBuffer(targetFile);
		// finally, visualization
		waveSurferRef.current?.loadBlob(targetFile);
	}
	// TODO: this might not be the cleanest way to refactor
	function handleIRLoaded(
		fileName: string,
		bytes: Uint8Array<ArrayBuffer>,
		metadata: IAudioMetadata
	) {
		const formatInfo = metadata.format;

		const irData: IrData = {
			name: fileName,
			rawBytes: [...bytes],
			// TODO: handle if these are undefined!
			lengthSeconds: formatInfo.duration!,
			numChannels: formatInfo.numberOfChannels!,
			sampleRate: formatInfo.sampleRate!,
		};
		setFileInfo(irData);
		sendToPlugin({
			type: 'irUpdate',
			data: irData,
		});
	}

	return (
		<>
			<input
				id='inp'
				type='file'
				onChange={onFileChange}
				className='hidden'
			/>
			<label
				htmlFor='inp'
				className='hover:cursor-pointer rounded-sm bg-zinc-500 p-1'
			>
				<h1>Load IR</h1>
			</label>
		</>
	);
}
