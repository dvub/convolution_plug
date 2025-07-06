import { GlobalParametersContext } from '@/contexts/GlobalParamsContext';
import { sendToPlugin } from '@/lib';
import { Parameter } from '@/lib/parameters';
import { useContext } from 'react';

export default function ParameterToggle(props: { parameter: Parameter }) {
	const { parameter } = props;
	const { parameters, setParameters } = useContext(GlobalParametersContext)!;
	const enabled = parameters[parameter];

	function handleClick() {
		let newValue = 0;
		if (enabled === 0) {
			newValue = 1;
		} else {
			newValue = 0;
		}

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
			<div className='w-6 h-6 border-2 border-black flex justify-center items-center'>
				{enabled === 1 && <div className='w-4 h-4 bg-black' />}
			</div>
		</button>
	);
}
