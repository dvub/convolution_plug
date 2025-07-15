import { dbToGain, gainToDb } from '@/lib/conversion';
import { NumericRange, gainSkewFactor, RangeType } from '@/lib/range';
import { Knob } from './knobs/Knob';
import { KNOB_DIGITS } from '@/lib/constants';

const GAIN_KNOB_SIZE = 15;

export default function GainControls() {
	return (
		<div className='w-[40%] h-full secondary rounded-sm py-5 flex flex-col justify-between'>
			<Knob
				parameter='dry_gain'
				label='Dry Gain'
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
			/>
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
			/>
		</div>
	);
}
