import { IrData } from '@/bindings/IrData';
import { Message } from '@/bindings/Message';

import { usePluginListener } from '@/hooks/usePluginListener';

import { useWaveform } from '@/hooks/useWaveform';
import { IRInput } from './IRInput';
import { useRef, useState } from 'react';
import { NormalizeControls } from './NormalizeControls';
import { ResampleControls } from './ResampleControls';
import { IrProcessingConfig } from '@/bindings/IrProcessingConfig';
import { sendToPlugin } from '@/lib';
import { IrInfoDisplay } from './IrInfoDisplay';

export function IRManager() {
	const containerRef = useRef(null);
	const waveSurferRef = useWaveform(containerRef);

	const [irData, setIrData] = useState<IrData | undefined>();
	const [irConfig, setIrConfig] = useState<IrProcessingConfig | undefined>();

	usePluginListener((message: Message) => {
		if (message.type !== 'initResponse') {
			return;
		}

		if (message.data.irData) {
			setIrData(message.data.irData);
		}
	});

	function handleDragOver(e: React.DragEvent<HTMLDivElement>) {
		e.preventDefault();
	}
	function handleFileDrop(event: React.DragEvent<HTMLDivElement>) {
		event.preventDefault();
		if (!event.dataTransfer || event.dataTransfer.files.length === 0) {
			return;
		}
		setIrFromFile(event.dataTransfer.files[0]);
	}

	// refactoring to this function lets us share behavior
	// between the button and the element which handles drag/drop

	// TODO: might want to work on nesting issues here
	function setIrFromFile(file: File) {
		const fileName = file.name;

		const reader = new FileReader();
		reader.onload = () => {
			const arrayBuffer = reader.result as ArrayBuffer;
			const bytes = new Uint8Array(arrayBuffer);

			const irData: IrData = {
				name: fileName,
				rawBytes: [...bytes],
			};
			setIrData(irData);
			sendToPlugin({
				type: 'irUpdate',
				data: irData,
			});
		};
		reader.readAsArrayBuffer(file);
		// finally, visualization
		waveSurferRef.current?.loadBlob(file);
	}

	return (
		<div
			className='w-full h-[35vh] flex gap-1'
			onDrop={handleFileDrop}
			onDragOver={handleDragOver}
		>
			<div
				ref={containerRef}
				className='w-[50%] h-full rounded-sm secondary'
			/>

			<div className='w-[50%] flex flex-col gap-1'>
				<IrInfoDisplay irData={irData} />

				<div className='h-full secondary rounded-sm p-1 text-xs flex flex-col justify-between text-center'>
					<div className='flex flex-col gap-1'>
						<NormalizeControls
							irConfig={irConfig!}
							setIrConfig={setIrConfig}
						/>
						<ResampleControls
							irConfig={irConfig!}
							setIrConfig={setIrConfig}
						/>
					</div>
					<IRInput setIrFromFile={setIrFromFile} />
				</div>
			</div>
		</div>
	);
}
