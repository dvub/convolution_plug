import { sendToPlugin } from '@/lib';

export function ResampleControls() {
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

	return (
		<div className='flex w-full items-center justify-between'>
			<h1 className='w-[33%]'>Resample</h1>
			<div className='flex w-[66%]'>
				<h1 className='w-[50%] bg-zinc-500 rounded-l-sm py-1'>Off</h1>
				<h1 className='w-[50%] bg-zinc-900 rounded-r-sm py-1'>
					In. SR
				</h1>
			</div>
		</div>
	);
}
