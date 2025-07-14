import { Message } from '@/bindings/Message';
import { MessageBusContext } from '@/contexts/MessageBusContext';
import { sendToPlugin } from '@/lib';
import { Parameter } from '@/lib/parameters';
import {
	useContext,
	useState,
	useEffect,
	Dispatch,
	SetStateAction,
} from 'react';

// TODO: improve return type
export function useParameter(
	parameter: Parameter
): [
	[number, (valueRaw: number) => void],
	[boolean, Dispatch<SetStateAction<boolean>>]
] {
	const messageBus = useContext(MessageBusContext)!;

	// TODO: is it necessary (or correct) for each instance to hold on to its own param map?
	// alternatively, we could probably use context..?
	const [paramMap, setParamMap] = useState<string[]>([]);
	const [value, setValue] = useState(0);
	// TODO: maybe something like isBlocked?
	// there might be other reasons other than dragging to block updates
	const [isDragging, setIsDragging] = useState(false);
	const [index, setIndex] = useState(0);

	useEffect(() => {
		// TODO: refactor
		const handlePluginMessage = (message: Message) => {
			if (message.type === 'initResponse') {
				const map = message.data.paramMap;
				const newIndex = map.indexOf(parameter);
				const newValue = message.data.initParams[newIndex].value;

				// console.log('Initializing...', parameter, newIndex, map);

				setParamMap(map);
				setIndex(newIndex);
				setValue(newValue);
			}

			if (message.type === 'parameterUpdate') {
				if (isDragging) {
					return;
				}

				if (index !== message.data.parameterIndex) {
					return;
				}

				setValue(message.data.value);
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

	return [
		[value, updateVal],
		[isDragging, setIsDragging],
	];
}
