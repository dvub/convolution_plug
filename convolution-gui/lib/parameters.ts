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
export function initParameters(): GlobalParameters {
	return {
		dry_gain: 0,
		wet_gain: 0,
		lowpass_enabled: 0,
		lowpass_freq: 0,
		lowpass_q: 0,
		bell_enabled: 0,
		bell_freq: 0,
		bell_q: 0,
		bell_gain: 0,
		highpass_enabled: 0,
		highpass_freq: 0,
		highpass_q: 0,
	};
}

export type Parameter = keyof GlobalParameters;
