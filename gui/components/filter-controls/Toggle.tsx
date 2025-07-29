import { DISABLED_OPACITY } from '@/lib/constants';

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
		<button
			onClick={handleClick}
			className='px-0.5 border-2 shadow-lg shadow-[#0d100f]/50 text-sm'
			style={{ opacity: enabled === 1 ? 1 : DISABLED_OPACITY }}
		>
			{label}
		</button>
	);
}
