// Conversion functions rewritten from NIH-plug
// https://github.com/robbert-vdh/nih-plug/blob/ffe9b61fcb0441c9d33f4413f5ebe7394637b21f/src/util.rs#L30

export function dbToGain(db: number) {
	return Math.pow(10, db * 0.05);
}

export function gainToDb(gain: number) {
	return Math.log10(Math.max(gain, 1e-6)) * 20.0;
}
