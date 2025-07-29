import { IrData } from '@/bindings/IrData';
import { parseBuffer } from 'music-metadata';
import { useEffect, useState } from 'react';

type IrMetadata = {
	name: string;
	length?: number;
	numChannels?: number;
	sampleRate?: number;
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

			setMeta({
				name: irData.name,
				length: metadata.duration,
				numChannels: metadata.numberOfChannels,
				sampleRate: metadata.sampleRate,
			});
		});

		return () => {};
	}, [irData]);

	if (!meta) {
		return <h1 className='secondary rounded-sm p-1'>No IR Loaded.</h1>;
	}

	/* https://stackoverflow.com/questions/1199352/smart-way-to-truncate-long-strings*/
	const shortName = meta.name.replace(/(.{20})..+/, '$1…');

	const duration = meta.length
		? `Length: ${meta.length.toFixed(3)}`
		: 'No length found';

	const channels = meta.numChannels
		? `${meta.numChannels} Channels`
		: 'No channel info found';

	const sampleRate = meta.sampleRate
		? `${meta.sampleRate} Hz`
		: 'No SR found';

	return (
		<div className='secondary rounded-sm p-1'>
			<h1 className='text-sm'>{shortName}</h1>
			<p className='text-xs'>
				{duration}
				<br />
				{channels}
				<br />
				{sampleRate}
				<br />
			</p>
		</div>
	);
}
