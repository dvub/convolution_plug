import { useState, useEffect } from 'react';
// import { Input } from '../ui/input';
import { KnobProps } from './Knob';

type KnobTextInputProps = Pick<KnobProps, 'maxValue' | 'minValue'> & {
	valueRaw: number;
	setVal(valueRaw: number): void;
	valueRawDisplayFn: (valueRaw: number) => string;
};

export function KnobTextInput(props: KnobTextInputProps) {
	const { maxValue, minValue, valueRaw, setVal, valueRawDisplayFn } = props;

	const [inputBuffer, setInputBuffer] = useState('');
	const [hasFocus, setHasFocus] = useState(false);
	useEffect(() => {
		if (!hasFocus) {
			setInputBuffer(valueRawDisplayFn(valueRaw));
		}
	}, [valueRaw, hasFocus]);

	function handleInputFinish() {
		if (inputBuffer === '' || isNaN(Number(inputBuffer))) {
			setVal(0);
			setInputBuffer('0');
		}
	}

	const handleKeyDown = (event: { key: string }) => {
		if (event.key == 'Enter') {
			handleInputFinish();
		}
	};

	// TODO:
	// handle -0
	const handleChange = (e: { target: { value: string } }) => {
		const input = e.target.value.trim();
		const castInput = input;

		// TODO:
		// there's probably a correct limit, i just don't know what to do
		if (input.length > 10) {
			return;
		}
		if (isNaN(castInput) && input !== '-') {
			// prevent the user from  typing in non-numbers (letters etc.)
			// EXCEPT FOR "-"
			// we do this in case the user wants to type a negative
			return;
		}

		// handle max and min
		if (castInput > maxValue) {
			setInputBuffer(maxValue.toString());
			return;
		}
		if (castInput < minValue) {
			setInputBuffer(minValue.toString());
			return;
		}
		setInputBuffer(input);

		if (!isNaN(castInput)) {
			setVal(castInput);
		}
	};
	const handleBlur = () => {
		handleInputFinish();
		setHasFocus(false);
	};

	const handleFocus = () => {
		setHasFocus(true);
	};

	return (
		<input
			value={inputBuffer}
			onChange={handleChange}
			onKeyDown={handleKeyDown}
			onBlur={handleBlur}
			onFocus={handleFocus}
			className='text-sm'
		/>
	);
}
