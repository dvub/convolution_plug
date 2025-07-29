import { hzThenKhz } from '@/lib/format';
import { Knob } from '../knobs/Knob';
import FilterColumn from './FilterColumn';
import ParameterToggle from './Toggle';

import {
	FILTER_KNOB_SIZE,
	DEFAULT_FREQ_RANGE,
	KNOB_DIGITS,
	DEFAULT_Q_RANGE,
	DEFAULT_Q,
} from '@/lib/constants';
import { useParameter } from '@/hooks/useParameter';

const DEFAULT_HIGHPASS_FREQ = 10;

export default function HighpassControls() {
	const [[isHighpassEnabled, setIsHighpassEnabled]] =
		useParameter('highpass_enabled');

	return (
		<FilterColumn>
			<ParameterToggle
				enabled={isHighpassEnabled}
				setEnabled={setIsHighpassEnabled}
				label='Lowcut'
			/>

			<Knob
				parameter='highpass_freq'
				label='Freq'
				size={FILTER_KNOB_SIZE}
				defaultValue={DEFAULT_HIGHPASS_FREQ}
				range={DEFAULT_FREQ_RANGE}
				valueRawDisplayFn={(x) => hzThenKhz(x, KNOB_DIGITS)}
				enabled={Boolean(isHighpassEnabled)}
			/>
			<Knob
				parameter='highpass_q'
				label='Q'
				size={FILTER_KNOB_SIZE}
				defaultValue={DEFAULT_Q}
				range={DEFAULT_Q_RANGE}
				valueRawDisplayFn={(x) => x.toFixed(KNOB_DIGITS)}
				enabled={Boolean(isHighpassEnabled)}
			/>
		</FilterColumn>
	);
}
