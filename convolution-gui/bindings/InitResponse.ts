// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { IrConfig } from './IrConfig';
import type { IrData } from './IrData';
import type { ParameterUpdate } from './ParameterUpdate';

export type InitResponse = {
	paramMap: Array<string>;
	initParams: Array<ParameterUpdate>;
	irData: IrData | null;
	config: IrConfig;
};
