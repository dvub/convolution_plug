'use client';
/**
 * Modified knob BASE -
 * source:
 * https://github.com/satelllte/react-knob-headless/blob/main/apps/docs/src/components/knobs/KnobBase.tsx
 */

import { useContext, useId, useState } from 'react';
import {
	KnobHeadless,
	KnobHeadlessLabel,
	KnobHeadlessOutput,
	useKnobKeyboardControls,
} from 'react-knob-headless';

import { KnobBaseThumb } from './KnobBaseThumb';

import { sendToPlugin } from '@/lib';
import { GlobalParametersContext } from '@/contexts/GlobalParamsContext';
import { NumericRange, RangeType } from '@/lib/range';
import { Parameter } from '@/lib/parameters';
import { DISABLED_OPACITY } from '@/lib/constants';
import { KnobGesture } from '@/bindings/KnobGesture';

// TODO: make this a parameter-only knob
// create a separate component for setting-related knobs if desired

// TODO: fix this whole component tbh

export type KnobProps = {
	defaultValue: number;
	// visual stuff
	label: string;
	size: number;
	range: NumericRange;

	// optional because knobs dont have to be parameters
	parameter?: Parameter;
	onChangeCallback?: (n: number) => void;
	value?: number;
	// TODO: make this work
	// stepFn: (valueRaw: number) => number;
	// stepLargerFn: (valueRaw: number) => number;

	valueRawDisplayFn: (valueRaw: number) => string;
	enabled?: boolean;
};

export function Knob(props: KnobProps) {
	const stepFn = () => 0;
	const stepLargerFn = () => 0;

	const {
		label,
		defaultValue: cosmeticDefaultValue,
		size,
		parameter,
		range: cosmeticRange,
		onChangeCallback,
		value,
		valueRawDisplayFn,
		enabled,
	} = props;
	// this value can be tweaked to adjust the feel of the knob
	const dragSensitivity = 0.006;

	const { parameters, setParameters } = useContext(GlobalParametersContext)!;

	// internally this is
	const internalMinValue = 0;
	const internalMaxValue = 1;
	const internalRange = new NumericRange(0, 1, 0.5, RangeType.Linear);
	const internalDefaultValue = cosmeticRange.normalize(cosmeticDefaultValue);

	// NOTE:
	// this is only important if we don't have a parameter supplied
	// TODO: dont set 0 to be default
	const [state, setState] = useState(0);

	let valueRaw = 0;

	if (parameter) {
		// TODO:
		// improve type safety here
		valueRaw = parameters[parameter];
		// console.log(valueRaw);
	} else if (value) {
		valueRaw = value;
	} else {
		valueRaw = state;
	}

	const mapTo01 = (x: number) => internalRange.normalize(x);
	const mapFrom01 = (x: number) => internalRange.unnormalize(x);

	const knobId = useId();
	const labelId = useId();

	// TODO:
	// probably make this work
	const keyboardControlHandlers = useKnobKeyboardControls({
		valueRaw: valueRaw,
		valueMin: internalMinValue,
		valueMax: internalMaxValue,

		step: stepFn(),
		stepLarger: stepLargerFn(),
		onValueRawChange: setVal,
	});

	function setVal(valueRaw: number) {
		if (onChangeCallback) {
			onChangeCallback(valueRaw);
		}

		// as previously mentioned, state is only used if a parameter isn't supplied
		// (and consequently, we can't use the params context as state)
		if (!parameter) {
			setState(valueRaw);
			return;
		}
		setParameters({
			...parameters,
			[parameter]: valueRaw,
		});

		// !!!!
		sendToPlugin({
			type: 'parameterUpdate',
			data: { parameterId: parameter, value: valueRaw },
		});
	}

	function resetValue() {
		setVal(internalDefaultValue);
	}

	const thumbProps = {
		value01: mapTo01(valueRaw),
		resetValue: resetValue,
	};

	const handlePointer = (gesture: KnobGesture) =>
		sendToPlugin({
			type: 'knobGesture',
			data: {
				parameter_id: parameter!,
				gesture,
			},
		});

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
				onValueRawChange={setVal}
				valueRaw={valueRaw}
				valueMin={internalMinValue}
				valueMax={internalMaxValue}
				valueRawDisplayFn={valueRawDisplayFn}
				// TODO: we probably need more here to make sure that every type of event is handled
				onPointerDown={() => handlePointer('startDrag')}
				onPointerUp={() => handlePointer('stopDrag')}
				// TODO:
				// what am i doing
				valueRawRoundFn={() => 0.0}
				{...keyboardControlHandlers}
			>
				<KnobBaseThumb {...thumbProps} />
			</KnobHeadless>

			<div>
				<KnobHeadlessOutput htmlFor={''} className='text-xs'>
					{valueRawDisplayFn(cosmeticRange.unnormalize(valueRaw))}
				</KnobHeadlessOutput>
			</div>
		</div>
	);
}
