import { hzThenKhz } from '@/lib/format';
import { Knob } from '../knobs/Knob';
import {
	DEFAULT_FREQ_RANGE,
	DEFAULT_Q,
	DEFAULT_Q_RANGE,
	FILTER_KNOB_SIZE,
	KNOB_DIGITS,
} from '@/lib/constants';
import FilterColumn from './FilterColumn';
import ParameterToggle from './Toggle';

import { useParameter } from '@/hooks/useParameter';

const DEFAULT_LOWPASS_FREQ = 22050;

export default function LowpassControls() {
	const [[isLowpassEnabled, setIsLowpassEnabled]] =
		useParameter('lowpass_enabled');

	return (
		<FilterColumn>
			<ParameterToggle
				enabled={isLowpassEnabled}
				setEnabled={setIsLowpassEnabled}
				label='Highcut'
			/>

			<Knob
				parameter='lowpass_freq'
				label='Freq'
				size={FILTER_KNOB_SIZE}
				defaultValue={DEFAULT_LOWPASS_FREQ}
				range={DEFAULT_FREQ_RANGE}
				valueRawDisplayFn={(x) => hzThenKhz(x, KNOB_DIGITS)}
				enabled={Boolean(isLowpassEnabled)}
			/>
			<Knob
				parameter='lowpass_q'
				label='Q'
				size={FILTER_KNOB_SIZE}
				defaultValue={DEFAULT_Q}
				range={DEFAULT_Q_RANGE}
				valueRawDisplayFn={(x) => x.toFixed(KNOB_DIGITS)}
				enabled={Boolean(isLowpassEnabled)}
			/>
		</FilterColumn>
	);
}
