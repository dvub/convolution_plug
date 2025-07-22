/**
 * Modified knob thumb -
 * original source:
 * https://github.com/satelllte/react-knob-headless/blob/main/apps/docs/src/components/knobs/KnobBaseThumb.tsx
 */

import { mapFrom01Linear } from '@dsp-ts/math';

type KnobBaseThumbProps = {
	readonly resetValue: () => void;
	readonly value01: number;
};

export function KnobBaseThumb(props: KnobBaseThumbProps) {
	const { value01, resetValue } = props;

	// when the element is double-clicked, we want to call whatever function was passed to reset the knob
	function handleClick(event: { detail: number }) {
		if (event.detail === 2) {
			resetValue();
		}
	}
	const angleMin = -145;
	const angleMax = 145;
	const angle = mapFrom01Linear(value01, angleMin, angleMax);
	return (
		<div
			className='absolute h-full w-full rounded-full bg-gradient-to-t from-[#8b7f98] to-[#ecf0ef] cursor-ns-resize border-[1px] border-[#0d100f] shadow-lg shadow-[#0d100f]/50'
			onClick={handleClick}
		>
			{/* thumb */}
			<div
				className='absolute h-full w-full'
				style={{ rotate: `${angle}deg` }}
			>
				<div className='absolute left-1/2 top-0 w-[5%] -translate-x-1/2 rounded-b-lg bg-[#0d100f] h-1/4' />
				{/*<p className='absolute top-[50%] text-center w-full -translate-y-1/2 font-black text-lg z-10 text-[#fcf3fc] opacity-25'></p>*/}
			</div>
		</div>
	);
}
