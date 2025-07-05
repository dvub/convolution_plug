import { Dispatch, SetStateAction } from 'react';
import { createContext } from 'react';

export const GlobalParametersContext = createContext<{
	parameters: GlobalParameters;
	setParameters: Dispatch<SetStateAction<GlobalParameters>>;
} | null>(null);

export interface GlobalParameters {
	dry_gain: number;
	wet_gain: number;
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
