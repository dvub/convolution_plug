'use client';

import { Message } from '@/bindings/Message';
import BellControls from '@/components/filter-controls/BellControls';
import HighpassControls from '@/components/filter-controls/HighpassControls';
import LowpassControls from '@/components/filter-controls/LowpassControls';
import GainControls from '@/components/GainControls';
import { IRManager } from '@/components/ir-controls/IRManager';
import { Resize } from '@/components/Resize';
import TopBar from '@/components/TopBar';
import { usePluginListener } from '@/hooks/usePluginListener';
import { sendToPlugin } from '@/lib';
import { useState, useEffect } from 'react';

// TODO: make mouse icons consistent
// for example, hovering over buttons

export default function Home() {
	// TODO: should improve loading behavior
	// right now this works by simply making the entire page have 0 opacity
	// if we do something conventional, such as a placeholder loading element
	// it prevents elements (e.g. knobs) from being loaded and receiving their initial values
	const [isLoading, setIsLoading] = useState(true);

	useEffect(() => {
		sendToPlugin({ type: 'init' });
	}, []);

	usePluginListener((event: Message) => {
		if (event.type === 'initResponse') {
			setIsLoading(false);
		}
	});

	return (
		<div style={{ opacity: isLoading ? 0 : 1 }}>
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
			<Resize />
		</div>
	);
}
