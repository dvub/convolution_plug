import { GlobalParametersContext } from '@/contexts/GlobalParamsContext';
import { sendToPlugin } from '@/lib';
import { DISABLED_OPACITY } from '@/lib/constants';

import { Parameter } from '@/lib/parameters';
import { useContext } from 'react';

export default function ParameterToggle(props: {
	parameter: Parameter;
	label: string;
}) {
	const { parameter, label } = props;
	const { parameters, setParameters } = useContext(GlobalParametersContext)!;
	const enabled = parameters[parameter];

	function handleClick() {
		const newValue = Number(!enabled);
		sendToPlugin({
			type: 'parameterUpdate',
			data: {
				parameterId: props.parameter,
				value: newValue,
			},
		});

		setParameters({
			...parameters,
			[parameter]: newValue,
		});
	}

	return (
		<button onClick={handleClick}>
			<div
				className='w-full h-full px-1 border-2 flex justify-center items-center shadow-lg shadow-[#0d100f]/50'
				style={{ opacity: enabled === 1 ? 1 : DISABLED_OPACITY }}
			>
				<h1>{label}</h1>
			</div>
		</button>
	);
}
