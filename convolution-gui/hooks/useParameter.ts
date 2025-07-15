import { Message } from '@/bindings/Message';
import { sendToPlugin } from '@/lib';
import { Parameter } from '@/lib/parameters';
import { useState, Dispatch, SetStateAction } from 'react';
import { useMessageSubscriber } from './useMessageSubscriber';

// TODO: improve return type
export function useParameter(
	parameter: Parameter
): [
	[number, (valueRaw: number) => void],
	[boolean, Dispatch<SetStateAction<boolean>>]
] {
	const [value, setValue] = useState(0);
	const [isBlocking, setIsBlocking] = useState(false);
	const [index, setIndex] = useState(0);

	useMessageSubscriber((message: Message) => {
		if (message.type === 'initResponse') {
			const map = message.data.paramMap;
			const newIndex = map.indexOf(parameter);
			if (newIndex === -1) {
				throw new Error('INVALID PARAMETER');
			}
			const newValue = message.data.initParams[newIndex].value;

			setIndex(newIndex);
			setValue(newValue);
		}
		if (message.type === 'parameterUpdate') {
			if (isBlocking) {
				return;
			}
			if (index !== message.data.parameterIndex) {
				return;
			}
			setValue(message.data.value);
		}
	});

	// TODO: use better naming
	function setValueFunction(valueRaw: number) {
		setValue(valueRaw);
		sendToPlugin({
			type: 'parameterUpdate',
			data: {
				parameterIndex: index,
				value: valueRaw,
			},
		});
	}

	return [
		[value, setValueFunction],
		[isBlocking, setIsBlocking],
	];
}
