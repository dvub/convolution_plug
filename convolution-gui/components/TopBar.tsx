import { GearIcon } from '@radix-ui/react-icons';

export default function TopBar() {
	return (
		<div className='h-[5vh] py-1'>
			<div className='h-full secondary rounded-sm px-1'>
				<div className='h-full flex justify-between items-center gap-2 align-middle rounded-sm'>
					<h1 className='text-xl'>CONVOLUTION</h1>
					<div className='w-full text-right text-xs'>
						<h1>dvub</h1>
					</div>

					<GearIcon className='w-8 h-8' />
				</div>
			</div>
		</div>
	);
}
