import { hzThenKhz } from '@/lib/format';
import { Knob } from '../knobs/Knob';
import {
	DEFAULT_FREQ_RANGE,
	DEFAULT_Q_RANGE,
	FILTER_KNOB_SIZE,
	KNOB_DIGITS,
} from '@/app/page';
import FilterColumn from './FilterColumn';
import ParameterToggle from '../Toggle';
import { GlobalParametersContext } from '@/contexts/GlobalParamsContext';
import { useContext } from 'react';

export default function LowpassControls() {
	const { parameters } = useContext(GlobalParametersContext)!;

	return (
		<FilterColumn>
			<div className='flex flex-col items-center gap-2'>
				<ParameterToggle parameter='lowpass_enabled' label='Lowpass' />
			</div>
			<Knob
				parameter='lowpass_freq'
				label='Freq'
				size={FILTER_KNOB_SIZE}
				defaultValue={10}
				range={DEFAULT_FREQ_RANGE}
				valueRawDisplayFn={(x) => hzThenKhz(x, KNOB_DIGITS)}
				enabled={Boolean(parameters.lowpass_enabled)}
			/>
			<Knob
				parameter='lowpass_q'
				label='Q'
				size={FILTER_KNOB_SIZE}
				defaultValue={0.1}
				range={DEFAULT_Q_RANGE}
				valueRawDisplayFn={(x) => x.toFixed(KNOB_DIGITS)}
				enabled={Boolean(parameters.lowpass_enabled)}
			/>
		</FilterColumn>
	);
}
