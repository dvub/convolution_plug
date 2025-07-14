import { hzThenKhz } from '@/lib/format';
import { Knob } from '../knobs/Knob';
import {
	DEFAULT_FREQ_RANGE,
	DEFAULT_Q_RANGE,
	FILTER_KNOB_SIZE,
	KNOB_DIGITS,
} from '@/lib/constants';
import FilterColumn from './FilterColumn';
import ParameterToggle from '../Toggle';

import { useParameter } from '@/hooks/useParameter';

export default function LowpassControls() {
	const [isLowpassEnabled, updateVal] = useParameter('lowpass_enabled');

	return (
		<FilterColumn>
			<ParameterToggle
				enabled={isLowpassEnabled}
				updateVal={updateVal}
				label='Highcut'
			/>

			<Knob
				parameter='lowpass_freq'
				label='Freq'
				size={FILTER_KNOB_SIZE}
				defaultValue={10}
				range={DEFAULT_FREQ_RANGE}
				valueRawDisplayFn={(x) => hzThenKhz(x, KNOB_DIGITS)}
				enabled={Boolean(isLowpassEnabled)}
			/>
			<Knob
				parameter='lowpass_q'
				label='Q'
				size={FILTER_KNOB_SIZE}
				defaultValue={0.1}
				range={DEFAULT_Q_RANGE}
				valueRawDisplayFn={(x) => x.toFixed(KNOB_DIGITS)}
				enabled={Boolean(isLowpassEnabled)}
			/>
		</FilterColumn>
	);
}
