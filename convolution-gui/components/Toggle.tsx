import { DISABLED_OPACITY } from '@/lib/constants';

// TODO: these props are not very clean, is there any other option?

// the reason we do this:
// calling useParameter() in the column AND in this toggle separately results in a delay
// e.g. user would click the toggle to disable highcut, and the knobs would become disabled after a delay
export default function ParameterToggle(props: {
	label: string;
	enabled: number;
	setEnabled: (valueRaw: number) => void;
}) {
	const { label, enabled, setEnabled } = props;

	function handleClick() {
		const newValue = Number(!enabled);
		setEnabled(newValue);
	}

	return (
		<button onClick={handleClick}>
			<div
				className='w-full h-full px-1 border-2 flex justify-center items-center shadow-lg shadow-[#0d100f]/50'
				style={{ opacity: enabled === 1 ? 1 : DISABLED_OPACITY }}
			>
				<h1>{label}</h1>
			</div>
		</button>
	);
}
