import { hzThenKhz } from '@/lib/format';
import { Knob } from '../knobs/Knob';
import FilterColumn from './FilterColumn';
import ParameterToggle from '../Toggle';
import { GlobalParametersContext } from '@/contexts/GlobalParamsContext';
import { useContext } from 'react';
import {
	FILTER_KNOB_SIZE,
	DEFAULT_FREQ_RANGE,
	KNOB_DIGITS,
	DEFAULT_Q_RANGE,
} from '@/lib/constants';

export default function HighpassControls() {
	const { parameters } = useContext(GlobalParametersContext)!;

	return (
		<FilterColumn>
			<ParameterToggle parameter='highpass_enabled' label='Lowcut' />

			<Knob
				parameter='highpass_freq'
				label='Freq'
				size={FILTER_KNOB_SIZE}
				defaultValue={10}
				range={DEFAULT_FREQ_RANGE}
				valueRawDisplayFn={(x) => hzThenKhz(x, KNOB_DIGITS)}
				enabled={Boolean(parameters.highpass_enabled)}
			/>
			<Knob
				parameter='highpass_q'
				label='Q'
				size={FILTER_KNOB_SIZE}
				defaultValue={0.1}
				range={DEFAULT_Q_RANGE}
				valueRawDisplayFn={(x) => x.toFixed(KNOB_DIGITS)}
				enabled={Boolean(parameters.highpass_enabled)}
			/>
		</FilterColumn>
	);
}
