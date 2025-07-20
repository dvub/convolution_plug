import { ChangeEvent } from 'react';

export function IRInput(props: { setIrFromFile: (file: File) => void }) {
	const { setIrFromFile } = props;

	function onFileChange(event: ChangeEvent<HTMLInputElement>) {
		event.preventDefault();
		if (!event.target.files || event.target.files.length === 0) {
			return;
		}
		setIrFromFile(event.target.files[0]);
	}

	return (
		<>
			<input
				id='inp'
				type='file'
				onChange={onFileChange}
				className='hidden'
			/>
			<label
				htmlFor='inp'
				className='hover:cursor-pointer rounded-sm bg-zinc-500 p-1'
			>
				<h1>Load IR</h1>
			</label>
		</>
	);
}
