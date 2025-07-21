import { IrProcessingConfig } from '@/bindings/IrProcessingConfig';
import { Message } from '@/bindings/Message';
import { useMessageSubscriber } from '@/hooks/useMessageSubscriber';
import { sendToPlugin } from '@/lib';
import { Dispatch, SetStateAction } from 'react';

export function ResampleControls(props: {
	irConfig: IrProcessingConfig;
	setIrConfig: Dispatch<SetStateAction<IrProcessingConfig | undefined>>;
}) {
	const { irConfig, setIrConfig } = props;

	useMessageSubscriber((message: Message) => {
		if (message.type === 'initResponse') {
			setIrConfig(message.data.config);
		}
	});

	const handleSwitchClick = () => {
		const newConfig: IrProcessingConfig = {
			...irConfig!,
			resample: !irConfig.resample,
		};
		setIrConfig(newConfig);
		sendToPlugin({
			type: 'irConfigUpdate',
			data: newConfig,
		});
	};

	return (
		<div className='flex w-full items-center justify-between'>
			<h1 className='w-[33%] text-left'>Resample</h1>
			<div className='flex w-[66%]'>
				<button
					className='w-[50%] rounded-l-sm py-1 bg-zinc-900'
					style={{
						opacity: irConfig?.resample ? '0.25' : '1',
					}}
					onClick={handleSwitchClick}
				>
					Off
				</button>
				<button
					className='w-[50%] rounded-r-sm py-1 bg-zinc-900'
					style={{
						opacity: irConfig?.resample ? '1' : '0.25',
					}}
					onClick={handleSwitchClick}
				>
					In. SR
				</button>
			</div>
		</div>
	);
}
