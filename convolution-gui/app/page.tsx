'use client';

import { MessageBus, MessageBusContext } from '@/contexts/MessageBusContext';
import { useEventDispatcher } from '@/hooks/useEventDispatcher';
import { useEffect, useState } from 'react';
import { GlobalParametersContext } from '@/contexts/GlobalParamsContext';
import { Message } from '@/bindings/Message';
import { sendToPlugin } from '@/lib';
import { IrLoader } from '@/components/IRLoader';
import LowpassControls from '@/components/filter-controls/LowpassControls';
import BellControls from '@/components/filter-controls/BellControls';
import HighpassControls from '@/components/filter-controls/HighpassControls';
import TopBar from '@/components/TopBar';
import GainControls from '@/components/GainControls';
import { initParameters } from '@/lib/parameters';

export default function Home() {
	const [messageBus] = useState(new MessageBus());
	const [parameters, setParameters] = useState(initParameters());
	const [paramMap, setParamMap] = useState<string[]>([]);

	useEventDispatcher(messageBus);

	useEffect(() => {
		sendToPlugin({ type: 'init' });
		// TODO: could probably put this in a hook tbh
		const handlePluginMessage = (event: Message) => {
			if (event.type === 'initResponse') {
				const map = event.data.paramMap;

				setParamMap(map);

				event.data.initParams.forEach((p) => {
					// TODO: refactor this, very similar to below code
					setParameters((prevState) => {
						const param = map[p.parameterIndex];
						const value = p.value;

						return {
							...prevState,
							[param]: value,
						};
					});
				});
			}
		};
		const unsubscribe = messageBus.subscribe(handlePluginMessage);
		return () => {
			unsubscribe();
		};
	}, [messageBus]);

	useEffect(() => {
		const handlePluginMessage = (event: Message) => {
			if (event.type === 'parameterUpdate') {
				setParameters((prevState) => {
					const param = paramMap[event.data.parameterIndex];
					return {
						...prevState,
						[param]: event.data.value,
					};
				});
			}
		};
		const unsubscribe = messageBus.subscribe(handlePluginMessage);
		return () => {
			unsubscribe();
		};
	}, [messageBus, paramMap]);

	return (
		<MessageBusContext.Provider value={messageBus}>
			<GlobalParametersContext.Provider
				value={{ parameters, setParameters, paramMap }}
			>
				<div className='flex-col h-[100vh] w-[100vw] px-1'>
					<TopBar />
					<IrLoader />
					<div className='flex gap-1 py-1 h-[60vh]'>
						<div className='w-[60%] h-full flex secondary rounded-sm p-1 gap-1 text-center'>
							<HighpassControls />
							<BellControls />
							<LowpassControls />
						</div>
						<div className='w-[40%] h-full'>
							<GainControls />
						</div>
					</div>
				</div>
			</GlobalParametersContext.Provider>
		</MessageBusContext.Provider>
	);
}
