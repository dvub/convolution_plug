import { NumericRange, skewFactor, RangeType } from './range';

export const DEFAULT_FREQ_RANGE = new NumericRange(
	10,
	22050,
	skewFactor(-2.5),
	RangeType.Skewed
);
export const DEFAULT_Q_RANGE = new NumericRange(
	0.1,
	18,
	skewFactor(-2),
	RangeType.Skewed
);
export const FILTER_KNOB_SIZE = 4;
export const KNOB_DIGITS = 2;

export const DISABLED_OPACITY = 0.25;
