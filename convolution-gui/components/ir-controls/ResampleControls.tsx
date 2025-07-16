import { IRConfig } from '@/bindings/IRConfig';
import { Message } from '@/bindings/Message';
import { useMessageSubscriber } from '@/hooks/useMessageSubscriber';
import { sendToPlugin } from '@/lib';
import { Dispatch, SetStateAction } from 'react';

export function ResampleControls(props: {
	irConfig: IRConfig;
	setIrConfig: Dispatch<SetStateAction<IRConfig | undefined>>;
}) {
	const { irConfig, setIrConfig } = props;

	useMessageSubscriber((message: Message) => {
		if (message.type === 'initResponse') {
			setIrConfig(message.data.config);
		}
	});
	/*
	function handleUpdate() {
		sendToPlugin({
			type: 'irConfigUpdate',
			data: {
				normalizeIrs: false,
				resample: true,
				normalizationLevel: 0,
			},
		});
	}
		*/
	const handleDisableClick = () => {
		const newConfig = { ...irConfig!, resample: false };
		setIrConfig(newConfig);
		sendToPlugin({
			type: 'irConfigUpdate',
			data: newConfig,
		});
	};
	const handleEnableClick = () => {
		const newConfig = { ...irConfig!, resample: true };
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
					onClick={handleDisableClick}
				>
					Off
				</button>
				<button
					className='w-[50%] rounded-r-sm py-1 bg-zinc-900'
					style={{
						opacity: irConfig?.resample ? '1' : '0.25',
					}}
					onClick={handleEnableClick}
				>
					In. SR
				</button>
			</div>
		</div>
	);
}
