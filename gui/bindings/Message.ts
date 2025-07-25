// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { InitResponse } from "./InitResponse";
import type { IrData } from "./IrData";
import type { IrProcessingConfig } from "./IrProcessingConfig";
import type { ParameterUpdate } from "./ParameterUpdate";

export type Message = { "type": "init" } | { "type": "parameterUpdate", "data": ParameterUpdate } | { "type": "irUpdate", "data": IrData } | { "type": "irConfigUpdate", "data": IrProcessingConfig } | { "type": "initResponse", "data": InitResponse };
