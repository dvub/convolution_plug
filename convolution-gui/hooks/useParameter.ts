import { Message } from '@/bindings/Message';
import { MessageBusContext } from '@/contexts/MessageBusContext';
import { sendToPlugin } from '@/lib';
import { Parameter } from '@/lib/parameters';
import { useContext, useState, useEffect } from 'react';

export function useParameter(parameter: Parameter) {
	const messageBus = useContext(MessageBusContext)!;

	const [paramMap, setParamMap] = useState<string[]>([]);
	const [value, setValue] = useState(0);
	const [isDragging, setIsDragging] = useState(false);
	const [index, setIndex] = useState(0);

	useEffect(() => {
		// TODO: refactor
		const handlePluginMessage = (event: Message) => {
			if (event.type === 'initResponse') {
				const map = event.data.paramMap;
				const newIndex = map.indexOf(parameter);
				const newValue = event.data.initParams[newIndex].value;

				console.log('Initializing...', parameter, newIndex, map);

				setParamMap(map);
				setIndex(newIndex);
				setValue(newValue);
			}

			if (event.type === 'parameterUpdate') {
				if (isDragging) {
					return;
				}

				// console.log(index, event.data.parameterIndex);
				if (index !== event.data.parameterIndex) {
					return;
				}
				console.log("NEW VAL:", event.data.value);

				setValue(event.data.value);
			}
		};
		const unsubscribe = messageBus.subscribe(handlePluginMessage);
		return () => {
			unsubscribe();
		};
	}, [messageBus, paramMap, isDragging, parameter, index]);

	function updateVal(valueRaw: number) {
		setValue(valueRaw);

		sendToPlugin({
			type: 'parameterUpdate',
			data: {
				parameterIndex: index,
				value: valueRaw,
			},
		});
	}

	return { value, updateVal, setIsDragging };
}
