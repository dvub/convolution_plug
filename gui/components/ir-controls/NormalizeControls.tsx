import { IrProcessingConfig } from '@/bindings/IrProcessingConfig';

import { sendToPlugin } from '@/lib';
import { Dispatch, SetStateAction } from 'react';

// TODO: refactor into specific numeric input component
export function NormalizeControls(props: {
	irConfig: IrProcessingConfig;
	setIrConfig: Dispatch<SetStateAction<IrProcessingConfig | undefined>>;
}) {
	const { irConfig, setIrConfig } = props;

	const handleSwitchClick = () => {
		const newConfig: IrProcessingConfig = {
			...irConfig!,
			normalize: !irConfig.normalize,
		};
		setIrConfig(newConfig);
		sendToPlugin({
			type: 'irConfigUpdate',
			data: newConfig,
		});
	};

	return (
		<div className='flex w-full items-center justify-between'>
			<h1 className='w-[33%] text-left'>Normalize</h1>
			<div className='flex w-[66%]'>
				<button
					className='w-[50%] rounded-l-sm py-1 bg-zinc-900'
					style={{
						opacity: irConfig?.normalize ? '0.25' : '1',
					}}
					onClick={handleSwitchClick}
				>
					Off
				</button>
				<button
					className='w-[50%] rounded-r-sm py-1 bg-zinc-900'
					style={{
						opacity: irConfig?.normalize ? '1' : '0.25',
					}}
					onClick={handleSwitchClick}
				>
					RMS
				</button>
			</div>
		</div>
	);
}
