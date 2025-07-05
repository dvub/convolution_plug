import { gainToDb } from './conversion';

// more code (mostly) ported from nih-plug
// thanks robbert

export function gainFormatter(x: number, digits: number) {
	const g = gainToDb(x).toFixed(digits);
	return `${g} dB`;
}

export function hzThenKhz(x: number, digits: number) {
	if (x < 1000) {
		const rounded = x.toFixed(digits);
		return `${rounded} Hz`;
	} else {
		const k = x / 1000;
		const rounded = k.toFixed(digits);
		return `${rounded} kHz`;
	}
}
