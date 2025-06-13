'use client';

import { MessageBus, MessageBusContext } from '@/contexts/MessageBusContext';
import { useEventDispatcher } from '@/hooks/useEventDispatcher';
import { useEffect, useState } from 'react';

import {
	GlobalParameters,
	GlobalParametersContext,
} from '@/contexts/GlobalParamsContext';

import { Message } from '@/bindings/Message';

import { sendToPlugin } from '@/lib';
import { Knob } from '@/components/knobs/Knob';
import { NormalisableRange } from '@/lib/utils';

export default function Home() {
	let size = { width: 0, height: 0 };

	const [messageBus] = useState(new MessageBus());
	const [parameters, setParameters] = useState<GlobalParameters>({
		gain: 0,
		dry_wet: 0,

		lowpass_enabled: false,
		lowpass_freq: 0,
		lowpass_q: 0,

		bell_enabled: false,
		bell_freq: 0,
		bell_q: 0,
		bell_gain: 0,

		highpass_enabled: false,
		highpass_freq: 0,
		highpass_q: 0,
	});

	useEventDispatcher(messageBus);

	useEffect(() => {
		sendToPlugin({ type: 'init' });

		const handlePluginMessage = (event: Message) => {
			// console.log(event);
			switch (event.type) {
				case 'parameterUpdate':
					setParameters((prevState) => {
						return {
							...prevState,
							[event.data.parameterId]: event.data.value,
						};
					});

					break;
			}
		};

		let cornerResizeMouseDown = false;
		let startPos = { x: 0, y: 0 };
		let startSize = { ...size };

		document
			.querySelector('.corner-resize')
			.addEventListener('mousedown', (e) => {
				cornerResizeMouseDown = true;
				startPos.x = e.clientX;
				startPos.y = e.clientY;
				startSize = { ...size };
			});

		window.addEventListener('mouseup', () => {
			cornerResizeMouseDown = false;
		});

		window.addEventListener('mousemove', (e) => {
			if (cornerResizeMouseDown) {
				const deltaX = e.clientX - startPos.x;
				const deltaY = e.clientY - startPos.y;
				const width = Math.max(100, startSize.width + deltaX);
				const height = Math.max(100, startSize.height + deltaY);
				size.width = width;
				size.height = height;
				sendToPlugin({
					type: 'resize',
					data: { width, height },
				});
			}
		});

		const unsubscribe = messageBus.subscribe(handlePluginMessage);

		return () => {
			unsubscribe();
		};
	}, []);

	return (
		<MessageBusContext.Provider value={messageBus}>
			<GlobalParametersContext.Provider
				value={{ parameters, setParameters }}
			>
				<h1>hello world {parameters.gain}</h1>
				<button
					onClick={() =>
						sendToPlugin({
							type: 'parameterUpdate',
							data: {
								parameterId: 'highpass_enabled',
								value: 'true',
							},
						})
					}
				>
					BUTTON
				</button>
				<Knob
					minValue={0}
					maxValue={1}
					defaultValue={0}
					label={'hi'}
					size={50}
					range={new NormalisableRange(0, 1, 0.5)}
					parameter='gain'
				></Knob>

				<div className='corner-resize'>
					<svg viewBox='0 0 10 10' width='10' height='10'>
						<path d='M 10 0 L 10 10 L 0 10 Z' fill='#ccc' />
					</svg>
				</div>
			</GlobalParametersContext.Provider>
		</MessageBusContext.Provider>
	);
}
