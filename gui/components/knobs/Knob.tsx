'use client';
/**
 * Modified knob BASE -
 * source:
 * https://github.com/satelllte/react-knob-headless/blob/main/apps/docs/src/components/knobs/KnobBase.tsx
 */

// TODO: make text input work
// TODO: create a separate component for settings knobs

import { useEffect, useId, useState } from 'react';
import {
	KnobHeadless,
	KnobHeadlessLabel,
	KnobHeadlessOutput,
	useKnobKeyboardControls,
} from 'react-knob-headless';

import { KnobBaseThumb } from './KnobBaseThumb';

import { NumericRange, RangeType } from '@/lib/range';

import { DISABLED_OPACITY } from '@/lib/constants';

import { useParameter } from '@/hooks/useParameter';
import { Parameter } from '@/lib/parameters';

// this value can be tweaked to adjust the feel of the knob
const SENSITIVITY = 0.006;

const NORMALIZED_MIN_VALUE = 0;
const NORMALIZED_MAX_VALUE = 1;
const NORMALIZED_CENTER = 0.5;
const NORMALIZED_RANGE = new NumericRange(
	NORMALIZED_MIN_VALUE,
	NORMALIZED_MAX_VALUE,
	NORMALIZED_CENTER,
	RangeType.Linear
);
const SMALL_STEP = 0.01; // 1%
const LARGE_STEP = 0.1; // 10%

// should we expose step functions in props?
export type KnobProps = {
	defaultValue: number;
	label?: string;
	size: number;
	range: NumericRange;
	parameter: Parameter;
	valueRawDisplayFn: (valueRaw: number) => string;
	enabled?: boolean;
};

export function Knob({
	label,
	size,
	defaultValue,
	range,
	parameter,
	valueRawDisplayFn,
	enabled,
}: KnobProps) {
	const knobId = useId();
	const labelId = useId();

	const [[value, setValue], [isDragging, setIsDragging]] =
		useParameter(parameter);

	const internalDefaultValue = range.normalize(defaultValue);
	const mapTo01 = (x: number) => NORMALIZED_RANGE.normalize(x);
	const mapFrom01 = (x: number) => NORMALIZED_RANGE.unnormalize(x);

	function handleKeyboardValueChange(
		newValueRaw: number,
		event: React.KeyboardEvent
	) {
		setValue(newValueRaw);
		if (event.type === 'keydown' && !isDragging) {
			setIsDragging(true);
		}
	}
	const keyboardControlHandlers = useKnobKeyboardControls({
		valueRaw: value,
		valueMin: NORMALIZED_MIN_VALUE,
		valueMax: NORMALIZED_MAX_VALUE,
		step: SMALL_STEP,
		stepLarger: LARGE_STEP,
		onValueRawChange: handleKeyboardValueChange,
	});

	function resetValue() {
		// TODO: reset to cosmetic initial value
		setValue(internalDefaultValue);
	}

	const thumbProps = {
		value01: mapTo01(value),
		resetValue: resetValue,
	};

	const style = {
		opacity:
			enabled === true || enabled === undefined ? 1 : DISABLED_OPACITY,
	};

	// broken lol
	const [unit, setUnit] = useState('vh');
	useEffect(() => {
		const newUnit = window.innerHeight > window.innerWidth ? 'vw' : 'vh';
		console.log(newUnit);
		setUnit(newUnit);
	}, []);

	return (
		<div className='flex flex-col items-center' style={style}>
			<KnobHeadlessLabel id={labelId} className='text-sm'>
				{label}
			</KnobHeadlessLabel>

			<KnobHeadless
				id={knobId}
				aria-labelledby={labelId}
				className={`relative outline-none`}
				style={{ width: `${size}${unit}`, height: `${size}${unit}` }}
				dragSensitivity={SENSITIVITY}
				mapTo01={mapTo01}
				mapFrom01={mapFrom01}
				onValueRawChange={setValue}
				valueRaw={value}
				valueMin={NORMALIZED_MIN_VALUE}
				valueMax={NORMALIZED_MAX_VALUE}
				valueRawDisplayFn={valueRawDisplayFn}
				onPointerDown={() => setIsDragging(true)}
				onPointerUp={() => setIsDragging(false)}
				onKeyUp={() => setIsDragging(false)}
				valueRawRoundFn={(x) => x}
				{...keyboardControlHandlers}
			>
				<KnobBaseThumb {...thumbProps} />
			</KnobHeadless>
			<div>
				<KnobHeadlessOutput htmlFor={''} className='text-xs'>
					{valueRawDisplayFn(range.unnormalize(value))}
				</KnobHeadlessOutput>
			</div>
		</div>
	);
}
