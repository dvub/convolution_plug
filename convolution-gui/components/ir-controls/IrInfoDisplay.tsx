import { IrData } from '@/bindings/IrData';

export function IrInfoDisplay(props: { irData: IrData | undefined }) {
	const { irData } = props;

	if (!irData) {
		return <h1>No IR Loaded.</h1>;
	}

	return (
		<div className='secondary rounded-sm p-1'>
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
		</div>
	);
}
