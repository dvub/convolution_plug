import { IrData } from '@/bindings/IrData';
import { parseBuffer } from 'music-metadata';
import { useEffect, useState } from 'react';

type IrMetadata = {
	name: string;
	length: number;
	numChannels: number;
	sampleRate: number;
};

export function IrInfoDisplay(props: { irData: IrData | undefined }) {
	const { irData } = props;

	const [meta, setMeta] = useState<IrMetadata | undefined>();

	useEffect(() => {
		if (!irData) {
			return;
		}
		parseBuffer(new Uint8Array(irData.rawBytes)).then((x) => {
			// TODO: handle when metadata is undefined
			const metadata = x.format;

			const shortName = irData.name.replace(/(.{20})..+/, '$1…');
			setMeta({
				name: shortName,
				length: metadata.duration!,
				numChannels: metadata.numberOfChannels!,
				sampleRate: metadata.sampleRate!,
			});
		});

		return () => {};
	}, [irData]);

	if (!meta) {
		return <h1 className='secondary rounded-sm p-1'>No IR Loaded.</h1>;
	}

	return (
		<div className='secondary rounded-sm p-1'>
			<h1 className='text-sm'>
				{/* https://stackoverflow.com/questions/1199352/smart-way-to-truncate-long-strings*/}
				{meta.name}
			</h1>
			<p className='text-xs'>
				Length: {meta.length.toFixed(3)}s
				<br />
				{meta.numChannels} Channels
				<br />
				{meta.sampleRate} Hz
				<br />
			</p>
		</div>
	);
}
