const NAME = 'ConvolveNTR';
const AUTHOR = 'dvub'; // :0

export default function TopBar() {
	return (
		<div className='h-[5vh] py-1'>
			<div className='h-full secondary px-1 flex justify-between items-center rounded-sm text-xl'>
				<h1>{NAME}</h1>
				<h1>{AUTHOR}</h1>
				{/* <GearIcon className='w-8 h-8' /> */}
			</div>
		</div>
	);
}
