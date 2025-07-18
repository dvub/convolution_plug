import { IRConfig } from '@/bindings/IRConfig';
import { Message } from '@/bindings/Message';
import { useMessageSubscriber } from '@/hooks/useMessageSubscriber';
import { sendToPlugin } from '@/lib';
import { Dispatch, SetStateAction } from 'react';

// TODO: refactor into specific numeric input component
export function NormalizeControls(props: {
	irConfig: IRConfig;
	setIrConfig: Dispatch<SetStateAction<IRConfig | undefined>>;
}) {
	const { irConfig, setIrConfig } = props;

	useMessageSubscriber((message: Message) => {
		if (message.type === 'initResponse') {
			setIrConfig(message.data.config);
		}
	});

	const handleSwitchClick = () => {
		const newConfig: IRConfig = {
			...irConfig!,
			normalizeIrs: !irConfig.normalizeIrs,
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
						opacity: irConfig?.normalizeIrs ? '0.25' : '1',
					}}
					onClick={handleSwitchClick}
				>
					Off
				</button>
				<button
					className='w-[50%] rounded-r-sm py-1 bg-zinc-900'
					style={{
						opacity: irConfig?.normalizeIrs ? '1' : '0.25',
					}}
					onClick={handleSwitchClick}
				>
					RMS
				</button>
			</div>
		</div>
	);
}
