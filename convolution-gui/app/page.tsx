'use client';

import { MessageBus, MessageBusContext } from '@/contexts/MessageBusContext';
import { useEventDispatcher } from '@/hooks/useEventDispatcher';
import { ChangeEvent, useEffect, useState } from 'react';

import {
	GlobalParameters,
	GlobalParametersContext,
} from '@/contexts/GlobalParamsContext';

import { Message } from '@/bindings/Message';

import { sendToPlugin } from '@/lib';
import { Knob } from '@/components/knobs/Knob';
import { dbToGain, gainToDb, NormalisableRange } from '@/lib/utils';

export default function Home() {
	const [messageBus] = useState(new MessageBus());
	const [parameters, setParameters] = useState<GlobalParameters>({
		gain: 0,
		dry_wet: 0,

		lowpass_enabled: 0,
		lowpass_freq: 0,
		lowpass_q: 0,

		bell_enabled: 0,
		bell_freq: 0,
		bell_q: 0,
		bell_gain: 0,

		highpass_enabled: 0,
		highpass_freq: 0,
		highpass_q: 0,
	});

	useEventDispatcher(messageBus);

	useEffect(() => {
		sendToPlugin({ type: 'init' });

		const handlePluginMessage = (event: Message) => {
			console.log(event);
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

		const unsubscribe = messageBus.subscribe(handlePluginMessage);

		return () => {
			unsubscribe();
		};
	}, []);

	function onFileChange(event: ChangeEvent<HTMLInputElement>) {
		const file = event.target;
		// TODO: should resampling be a feature? if so, shouuld it be through the Web Audio API?
		// (or e.g. we implement it ourselves somehow)

		const reader = new FileReader();

		reader.onload = () => {
			const buffer = reader.result as ArrayBuffer;
			const ctx = new AudioContext({
				sampleRate: getSampleRate(buffer).sampleRate,
			});

			ctx.decodeAudioData(buffer, (decoded) => {
				const typedSamples = decoded.getChannelData(0);

				console.log(typedSamples);

				sendToPlugin({
					type: 'slotUpdate',
					data: [...typedSamples],
				});
			});
		};

		reader.readAsArrayBuffer(file.files![0]);

		{
		}
	}

	return (
		<MessageBusContext.Provider value={messageBus}>
			<GlobalParametersContext.Provider
				value={{ parameters, setParameters }}
			>
				<input type='file' onChange={onFileChange}></input>

				<Knob
					parameter='gain'
					label={'Gain'}
					size={50}
					//
					cosmeticDefaultValue={dbToGain(0)}
					cosmeticRange={
						new NormalisableRange(
							dbToGain(-30),
							dbToGain(30),
							dbToGain(0)
						)
					}
					valueRawDisplayFn={(x) => {
						let g = gainToDb(x).toFixed(2);
						// TODO: make this not be really scuffed
						if (g === '-0.00') {
							g = '0.00';
						}
						return `${g} dB`;
					}}
				></Knob>
			</GlobalParametersContext.Provider>
		</MessageBusContext.Provider>
	);
}

// TODO: move this elsewhere

// get me out of here
// https://github.com/WebAudio/web-audio-api/issues/30#issuecomment-1090167849
function getSampleRate(arrayBuffer: ArrayBuffer) {
	const view = new DataView(arrayBuffer);
	const chunkCellSize = 4;

	const getChunkName = (newOffset: number) =>
		String.fromCharCode.apply(null, [
			...new Int8Array(
				arrayBuffer.slice(newOffset, newOffset + chunkCellSize)
			),
		]);

	const isWave = getChunkName(0).includes('RIFF');
	if (!isWave) return { sampleRate: 0, bitsPerSample: 0 };

	let offset = 12;
	let chunkName = getChunkName(offset);
	let chunkSize = 0;

	while (!chunkName.includes('fmt')) {
		chunkSize = view.getUint32(offset + chunkCellSize, true);
		offset += 2 * chunkCellSize + chunkSize; // name cell + data_size cell + data size
		chunkName = getChunkName(offset);

		if (offset > view.byteLength)
			throw new Error("Couldn't find sampleRate.");
	}

	const sampleRateOffset = 12;
	const bitsPerSampleOffset = 22;

	const sampleRate = view.getUint32(offset + sampleRateOffset, true);
	const bitsPerSample = view.getUint16(offset + bitsPerSampleOffset, true);

	return { sampleRate, bitsPerSample };
}
