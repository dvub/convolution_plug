"use client";
/**
 * Modified knob BASE -
 * source:
 * https://github.com/satelllte/react-knob-headless/blob/main/apps/docs/src/components/knobs/KnobBase.tsx
 */

import { useContext, useId, useState } from "react";
import {
  KnobHeadless,
  KnobHeadlessLabel,
  useKnobKeyboardControls,
} from "react-knob-headless";

import { KnobBaseThumb } from "./KnobBaseThumb";

import { NormalisableRange } from "../../lib/utils";
import { sendToPlugin } from "@/lib";
import { GlobalParametersContext } from "@/contexts/GlobalParamsContext";
import { KnobTextInput } from "./KnobTextInput";

/*
type KnobHeadlessProps = React.ComponentProps<typeof KnobHeadless>;

Pick<
	KnobHeadlessProps,
	'valueMin' | 'valueMax' | 'orientation'
> & {...}
*/
export type KnobProps = {
  // basics
  minValue: number;
  maxValue: number;
  defaultValue: number;
  // visual stuff
  label: string;
  size: number;
  range: NormalisableRange;

  // optional because knobs dont have to be parameters
  parameter?: string;
  onChangeCallback?: (n: number) => void;
  value?: number;
  // TODO: make this work
  // stepFn: (valueRaw: number) => number;
  // stepLargerFn: (valueRaw: number) => number;

  valueRawDisplayFn: (valueRaw: number) => string;
};

export function Knob(props: KnobProps) {
  const stepFn = () => 0;
  const stepLargerFn = () => 0;

  const {
    label,
    defaultValue,
    minValue,
    maxValue,
    size,
    parameter,
    range,
    onChangeCallback,
    value,
    valueRawDisplayFn,
  } = props;
  // this value can be tweaked to adjust the feel of the knob
  const dragSensitivity = 0.006;

  const { parameters, setParameters } = useContext(GlobalParametersContext)!;

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

  const mapTo01 = (x: number) => range.mapTo01(x);
  const mapFrom01 = (x: number) => range.mapFrom01(x);

  const knobId = useId();
  const labelId = useId();

  // TODO:
  // probably make this work
  const keyboardControlHandlers = useKnobKeyboardControls({
    valueRaw: valueRaw,
    valueMin: minValue,
    valueMax: maxValue,

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
      type: "parameterUpdate",
      data: { parameterId: parameter, value: String(valueRaw) },
    });
  }

  function resetValue() {
    setVal(defaultValue);
  }

  const thumbProps = {
    value01: mapTo01(valueRaw),
    resetValue: resetValue,
  };

  return (
    <div className={"flex items-center m-3"}>
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
        valueMin={minValue}
        valueMax={maxValue}
        valueRawDisplayFn={valueRawDisplayFn}
        valueRawRoundFn={() => 0.0}
        {...keyboardControlHandlers}
      >
        <KnobBaseThumb {...thumbProps} />
      </KnobHeadless>

      <div className="mx-3">
        <KnobHeadlessLabel id={labelId} className="text-md">
          {label}
        </KnobHeadlessLabel>

        <KnobTextInput
          minValue={minValue}
          maxValue={maxValue}
          valueRaw={valueRaw}
          setVal={setVal}
          valueRawDisplayFn={valueRawDisplayFn}
        />
      </div>
    </div>
  );
}
