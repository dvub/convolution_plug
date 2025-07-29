import {
	DEFAULT_FREQ_RANGE,
	DEFAULT_Q,
	DEFAULT_Q_RANGE,
	FILTER_KNOB_SIZE,
	KNOB_DIGITS,
} from '@/lib/constants';
import { dbToGain } from '@/lib/conversion';
import { hzThenKhz, gainFormatter } from '@/lib/format';
import { NumericRange, gainSkewFactor, RangeType } from '@/lib/range';
import { Knob } from '../knobs/Knob';
import FilterColumn from './FilterColumn';
import ParameterToggle from './Toggle';
import { useParameter } from '@/hooks/useParameter';

const DEFAULT_BELL_FREQ = 10;

export default function BellControls() {
	const [[isBellEnabled, setIsBellEnabled]] = useParameter('bell_enabled');

	return (
		<FilterColumn>
			<ParameterToggle
				enabled={isBellEnabled}
				setEnabled={setIsBellEnabled}
				label='Bell'
			/>

			<Knob
				parameter='bell_freq'
				label='Freq'
				size={FILTER_KNOB_SIZE}
				defaultValue={DEFAULT_BELL_FREQ}
				range={DEFAULT_FREQ_RANGE}
				valueRawDisplayFn={(x) => hzThenKhz(x, KNOB_DIGITS)}
				enabled={Boolean(isBellEnabled)}
			/>
			<Knob
				parameter='bell_q'
				label='Q'
				size={FILTER_KNOB_SIZE}
				defaultValue={DEFAULT_Q}
				range={DEFAULT_Q_RANGE}
				valueRawDisplayFn={(x) => x.toFixed(KNOB_DIGITS)}
				enabled={Boolean(isBellEnabled)}
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
				enabled={Boolean(isBellEnabled)}
			/>
		</FilterColumn>
	);
}
