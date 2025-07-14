'use client';
/**
 * Modified knob BASE -
 * source:
 * https://github.com/satelllte/react-knob-headless/blob/main/apps/docs/src/components/knobs/KnobBase.tsx
 */

// TODO: allow user to type in values, maybe through some sort of form, i don't know

import { useId } from 'react';
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

// TODO: make this a parameter-only knob
// create a separate component for setting-related knobs if desired

// TODO: fix this whole component tbh

export type KnobProps = {
	defaultValue: number;
	// visual stuff
	label: string;
	size: number;
	range: NumericRange;
	parameter: Parameter;
	// onChangeCallback?: (n: number) => void;

	// stepFn: (valueRaw: number) => number;
	// stepLargerFn: (valueRaw: number) => number;

	valueRawDisplayFn: (valueRaw: number) => string;
	enabled?: boolean;
};

export function Knob(props: KnobProps) {
	const {
		label,
		defaultValue: cosmeticDefaultValue,
		size,
		parameter,
		range: cosmeticRange,

		valueRawDisplayFn,
		enabled,
	} = props;

	// this value can be tweaked to adjust the feel of the knob
	const dragSensitivity = 0.006;

	const [value, updateVal, setIsDragging] = useParameter(parameter);

	// TODO: rewrite internal range
	const internalMinValue = 0;
	const internalMaxValue = 1;
	const internalRange = new NumericRange(0, 1, 0.5, RangeType.Linear);
	const internalDefaultValue = cosmeticRange.normalize(cosmeticDefaultValue);
	const mapTo01 = (x: number) => internalRange.normalize(x);
	const mapFrom01 = (x: number) => internalRange.unnormalize(x);

	const knobId = useId();
	const labelId = useId();

	// TODO: probably make this work
	const stepFn = () => 0;
	const stepLargerFn = () => 0;

	const keyboardControlHandlers = useKnobKeyboardControls({
		valueRaw: value,
		valueMin: internalMinValue,
		valueMax: internalMaxValue,
		step: stepFn(),
		stepLarger: stepLargerFn(),
		onValueRawChange: updateVal,
	});

	function resetValue() {
		updateVal(internalDefaultValue);
	}

	const thumbProps = {
		value01: mapTo01(value),
		resetValue: resetValue,
	};

	return (
		<div
			className='flex flex-col items-center text-xs'
			style={{
				opacity:
					enabled === true || enabled === undefined
						? 1
						: DISABLED_OPACITY,
			}}
		>
			<KnobHeadlessLabel id={labelId} className='text-sm'>
				{label}
			</KnobHeadlessLabel>
			<KnobHeadless
				id={knobId}
				aria-labelledby={labelId}
				className={`relative outline-none`}
				style={{ width: `${size}px`, height: `${size}px` }}
				dragSensitivity={dragSensitivity}
				mapTo01={mapTo01}
				mapFrom01={mapFrom01}
				onValueRawChange={updateVal}
				valueRaw={value}
				valueMin={internalMinValue}
				valueMax={internalMaxValue}
				valueRawDisplayFn={valueRawDisplayFn}
				// TODO: we probably need more here to make sure that every type of event is handled
				onPointerDown={() => setIsDragging(true)}
				onPointerUp={() => setIsDragging(false)}
				// TODO: figure out what this does HAHA
				valueRawRoundFn={() => 0.0}
				{...keyboardControlHandlers}
			>
				<KnobBaseThumb {...thumbProps} />
			</KnobHeadless>

			<div>
				<KnobHeadlessOutput htmlFor={''} className='text-xs'>
					{valueRawDisplayFn(cosmeticRange.unnormalize(value))}
				</KnobHeadlessOutput>
			</div>
		</div>
	);
}
