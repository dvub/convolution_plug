import { Dispatch, SetStateAction } from 'react';
import { createContext } from 'react';

export const GlobalParametersContext = createContext<{
	parameters: GlobalParameters;
	setParameters: Dispatch<SetStateAction<GlobalParameters>>;
} | null>(null);

export interface GlobalParameters {
	gain: number;
	dry_wet: number;
	// LP
	lowpass_enabled: number;
	lowpass_freq: number;
	lowpass_q: number;
	// BELL
	bell_enabled: number;
	bell_freq: number;
	bell_q: number;
	bell_gain: number;
	// HP
	highpass_enabled: number;
	highpass_freq: number;
	highpass_q: number;
}

export type Parameter = keyof GlobalParameters;
