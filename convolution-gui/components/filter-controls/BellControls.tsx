import {
	DEFAULT_FREQ_RANGE,
	DEFAULT_Q_RANGE,
	FILTER_KNOB_SIZE,
	KNOB_DIGITS,
} from '@/app/page';
import { dbToGain } from '@/lib/conversion';
import { hzThenKhz, gainFormatter } from '@/lib/format';
import { NumericRange, gainSkewFactor, RangeType } from '@/lib/range';
import { Knob } from '../knobs/Knob';
import FilterColumn from './FilterColumn';
import ParameterToggle from '../Toggle';

export default function BellControls() {
	return (
		<FilterColumn>
			<div className='flex flex-col items-center gap-2'>
				<h1>Bell</h1>
				<ParameterToggle parameter='bell_enabled' />
			</div>

			<Knob
				parameter='bell_freq'
				label='Freq'
				size={FILTER_KNOB_SIZE}
				defaultValue={10}
				range={DEFAULT_FREQ_RANGE}
				valueRawDisplayFn={(x) => hzThenKhz(x, KNOB_DIGITS)}
			/>
			<Knob
				parameter='bell_q'
				label='Q'
				size={FILTER_KNOB_SIZE}
				defaultValue={0.1}
				range={DEFAULT_Q_RANGE}
				valueRawDisplayFn={(x) => x.toFixed(KNOB_DIGITS)}
			/>
			<Knob
				parameter='bell_gain'
				label='Gain'
				size={FILTER_KNOB_SIZE}
				defaultValue={dbToGain(0)}
				range={
					new NumericRange(
						dbToGain(-15),
						dbToGain(15),
						gainSkewFactor(-15, 15),
						RangeType.Skewed
					)
				}
				valueRawDisplayFn={(x) => gainFormatter(x, KNOB_DIGITS)}
			/>
		</FilterColumn>
	);
}
