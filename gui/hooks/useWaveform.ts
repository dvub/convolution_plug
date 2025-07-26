import { Message } from '@/bindings/Message';
import { useRef, useEffect, RefObject } from 'react';
import WaveSurfer from 'wavesurfer.js';
import { usePluginListener } from './usePluginListener';

export function useWaveform(containerRef: RefObject<HTMLElement | null>) {
	const waveSurferRef = useRef<WaveSurfer | null>(null);

	useEffect(() => {
		const waveSurfer = WaveSurfer.create({
			container: containerRef.current!,
			height: 'auto',
			waveColor: '#ecf0ef',
			normalize: true,
			cursorWidth: 0,
			interact: false,
		});

		waveSurfer.on('init', () => {
			waveSurferRef.current = waveSurfer;
		});
	}, [containerRef]);

	usePluginListener((event: Message) => {
		if (event.type !== 'initResponse') {
			return;
		}

		const irData = event.data.irData;
		if (!irData) {
			return;
		}

		const blob = new Blob([new Uint8Array(irData.rawBytes)], {
			type: 'wav',
		});
		waveSurferRef.current!.loadBlob(blob);
	});

	return waveSurferRef;
}
