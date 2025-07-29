import { Message } from '@/bindings/Message';
import { sendToPlugin } from '@/lib';
import { Parameter } from '@/lib/parameters';
import { useState, Dispatch, SetStateAction, useCallback } from 'react';
import { usePluginListener } from './usePluginListener';

// TODO: improve return type
export function useParameter(
	parameter: Parameter
): [
	[number, (valueRaw: number) => void],
	[boolean, Dispatch<SetStateAction<boolean>>]
] {
	const [value, setValue] = useState(0);
	const [isBlocking, setIsBlocking] = useState(false);

	const handleMessage = useCallback(
		(message: Message) => {
			if (message.type === 'initResponse') {
				const matchedParameterUpdate = message.data.initParams.filter(
					(x) => x.parameterId === parameter
				)[0];
				setValue(matchedParameterUpdate.value);
			}
			if (message.type === 'parameterUpdate') {
				if (isBlocking) {
					return;
				}
				for (const parameterUpdate of message.data) {
					if (parameter !== parameterUpdate.parameterId) {
						continue;
					}
					setValue(parameterUpdate.value);
				}
			}
		},
		[isBlocking, parameter]
	);
	usePluginListener(handleMessage);

	// TODO: use better naming

	// TODO: instead of having these be arrays with single elements,
	// what if we somehow aggregated updates from multiple parameters?
	function setValueFunction(valueRaw: number) {
		setValue(valueRaw);
		sendToPlugin({
			type: 'parameterUpdate',
			data: [
				{
					parameterId: parameter,
					value: valueRaw,
				},
			],
		});
	}

	return [
		[value, setValueFunction],
		[isBlocking, setIsBlocking],
	];
}
