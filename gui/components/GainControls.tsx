import { dbToGain, gainToDb } from '@/lib/conversion';
import { NumericRange, gainSkewFactor, RangeType } from '@/lib/range';
import { Knob } from './knobs/Knob';
import { DISABLED_OPACITY, KNOB_DIGITS } from '@/lib/constants';
import { useParameter } from '@/hooks/useParameter';
import { SpeakerLoudIcon, SpeakerOffIcon } from '@radix-ui/react-icons';

const GAIN_KNOB_SIZE = 7;

export default function GainControls() {
	return (
		<div className='w-[40%] h-full secondary rounded-sm py-5 flex flex-col justify-center items-center'>
			<div>
				<DryKnob />
				<Knob
					parameter='wet_gain'
					label='Wet Gain'
					size={GAIN_KNOB_SIZE}
					defaultValue={dbToGain(0)}
					range={
						new NumericRange(
							dbToGain(-40),
							dbToGain(40),
							gainSkewFactor(-40, 40),
							RangeType.Skewed
						)
					}
					valueRawDisplayFn={(x) => {
						const g = gainToDb(x).toFixed(KNOB_DIGITS);
						return `${g} dB`;
					}}
				/>{' '}
			</div>
		</div>
	);
}
// TODO: refactor into separate files?
function DryKnob() {
	const [[value, setValue]] = useParameter('dry_enabled');

	const style = { opacity: value === 1 ? 1.0 : DISABLED_OPACITY };

	return (
		<div className='flex flex-col items-center'>
			<div className='flex items-center gap-3'>
				<h1 className='text-sm' style={style}>
					Dry Gain
				</h1>
				<DryEnabledButton value={value} setValue={setValue} />
			</div>
			<Knob
				parameter='dry_gain'
				size={GAIN_KNOB_SIZE}
				defaultValue={dbToGain(0)}
				range={
					new NumericRange(
						dbToGain(-30),
						dbToGain(30),
						gainSkewFactor(-30, 30),
						RangeType.Skewed
					)
				}
				valueRawDisplayFn={(x) => {
					const g = gainToDb(x).toFixed(KNOB_DIGITS);
					return `${g} dB`;
				}}
				enabled={Boolean(value)}
			/>
		</div>
	);
}

function DryEnabledButton(props: {
	value: number;
	setValue: (valueRaw: number) => void;
}) {
	const { value, setValue } = props;

	function handleDryEnabledClick() {
		setValue(Number(!value));
	}

	return (
		<button onClick={handleDryEnabledClick} className='hover:cursor'>
			{value === 1 ? <SpeakerLoudIcon /> : <SpeakerOffIcon />}
		</button>
	);
}
