export type Parameter =
	| 'dry_gain'
	| 'wet_gain'
	| 'dry_enabled'
	// LP
	| 'lowpass_enabled'
	| 'lowpass_freq'
	| 'lowpass_q'
	// BELL
	| 'bell_enabled'
	| 'bell_freq'
	| 'bell_q'
	| 'bell_gain'
	// HP
	| 'highpass_enabled'
	| 'highpass_freq'
	| 'highpass_q';
