import { GearIcon } from '@radix-ui/react-icons';

export default function TopBar() {
	return (
		<div className='h-[5vh] bg-zinc-700 py-1 rounded-sm'>
			<div className='flex justify-between gap-2 items-center h-[5vh] bg-zinc-500 rounded-sm'>
				<h1>CONVOLUTION</h1>
				<div className='w-full text-right'>
					<h1>dvub</h1>
				</div>

				<GearIcon />
			</div>
		</div>
	);
}
