// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { IrData } from "./IrData";
import type { ParameterUpdate } from "./ParameterUpdate";

export type Message = { "type": "init" } | { "type": "resize", "data": { width: number, height: number, } } | { "type": "parameterUpdate", "data": ParameterUpdate } | { "type": "irUpdate", "data": IrData };
