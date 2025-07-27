'use client';

import { Message } from '@/bindings/Message';
import BellControls from '@/components/filter-controls/BellControls';
import HighpassControls from '@/components/filter-controls/HighpassControls';
import LowpassControls from '@/components/filter-controls/LowpassControls';
import GainControls from '@/components/GainControls';
import { IRManager } from '@/components/ir-controls/IRManager';
import TopBar from '@/components/TopBar';
import { usePluginListener } from '@/hooks/usePluginListener';
import { sendToPlugin } from '@/lib';
import { useState, useEffect } from 'react';

// TODO: make mouse icons consistent
// for example, hovering over buttons

// TODO: really need to improve resizing
// - double click should reset to original size
// - initresponse should include window size, maybe?
// - maybe add resizing to the sides of the window instead of just the corner?

export default function Home() {
	// TODO: possibly change/fix loading behavior?
	// right now this works by simply making the entire page have 0 opacity
	// if we do something conventional, such as a placeholder loading element
	// it prevents elements (e.g. knobs) from being loaded and receiving their initial values
	const [isLoading, setIsLoading] = useState(true);

	useEffect(() => {
		console.log('PLUGIN:', window.plugin);

		sendToPlugin({ type: 'init' });
	}, []);

	usePluginListener((event: Message) => {
		console.log(event);
		if (event.type === 'initResponse') {
			setIsLoading(false);
		}
	});

	const [startPos, setStartPos] = useState({ x: 0, y: 0 });
	const [mouseDown, setMouseDown] = useState(false);
	const [size, setSize] = useState({ width: 600, height: 600 });
	const [iSize, setISize] = useState({ width: 0, height: 0 });

	function handleResizeDown(e: React.PointerEvent<HTMLDivElement>) {
		setMouseDown(true);
		setStartPos({ x: e.clientX, y: e.clientY });

		setISize(size);
	}
	function handleResizeUp() {
		setMouseDown(false);
	}
	function handleResizeMove(e: React.PointerEvent<HTMLDivElement>) {
		if (!mouseDown) {
			return;
		}

		const deltaX = e.clientX - startPos.x;
		const deltaY = e.clientY - startPos.y;

		const width = Math.max(100, iSize.width + deltaX);
		const height = Math.max(100, iSize.height + deltaY);

		setSize({ width, height });
		sendToPlugin({
			type: 'resize',
			data: {
				width,
				height,
			},
		});
	}

	return (
		<div
			style={{ opacity: isLoading ? 0 : 1 }}
			onPointerUp={handleResizeUp}
			onPointerMove={handleResizeMove}
		>
			<TopBar />
			<IRManager />
			<div className='flex gap-1 py-1 h-[60vh]'>
				<div className='w-[60%] flex secondary rounded-sm p-1 gap-1'>
					<HighpassControls />
					<BellControls />
					<LowpassControls />
				</div>
				<GainControls />
			</div>
			<div
				className='corner-resize absolute bottom-0 right-0 h-10 w-10 bg-red-500'
				onPointerDown={(e) => handleResizeDown(e)}
			/>
		</div>
	);
}
