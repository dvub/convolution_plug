import { dbToGain } from './conversion';

export enum RangeType {
	Linear,
	Skewed,
}

// https://github.com/robbert-vdh/nih-plug/blob/ecfd6322c776d9f0373d31758976c2353c162737/src/params/range.rs#L39

export class NumericRange {
	private rangeType: RangeType;
	private min: number;
	private max: number;
	private factor: number;

	constructor(
		min: number,
		max: number,
		factor: number,
		rangeType: RangeType
	) {
		this.min = min;
		this.max = max;
		this.factor = factor;
		this.rangeType = rangeType;
	}

	public normalize(plain: number) {
		const min = this.min;
		const max = this.max;
		const factor = this.factor;

		switch (this.rangeType) {
			case RangeType.Linear:
				return (clamp(plain, min, max) - min) / (max - min);

			case RangeType.Skewed:
				return Math.pow(
					(Math.min(Math.max(plain, min), max) - min) / (max - min),
					factor
				);
		}
	}
	public unnormalize(normalized: number) {
		const min = this.min;
		const max = this.max;
		const factor = this.factor;

		const norm = clamp(normalized, 0, 1);

		switch (this.rangeType) {
			case RangeType.Linear:
				return norm * (max - min) + min;
			case RangeType.Skewed:
				return Math.pow(norm, 1 / factor) * (max - min) + min;
		}
	}
}

function clamp(number: number, min: number, max: number) {
	return Math.max(min, Math.min(number, max));
}

export function skewFactor(factor: number) {
	return Math.pow(2, factor);
}
export function gainSkewFactor(minDb: number, maxDb: number): number {
	const minGain = dbToGain(minDb);
	const maxGain = dbToGain(maxDb);
	const middleDb = (maxDb + minDb) / 2;
	const middleGain = dbToGain(middleDb);

	return (
		Math.log(0.5) / Math.log((middleGain - minGain) / (maxGain - minGain))
	);
}
