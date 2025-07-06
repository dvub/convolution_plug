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

	useEventDispatcher(messageBus);
	// TODO: refactor
	useEffect(() => {
		sendToPlugin({ type: 'init' });
		const handlePluginMessage = (event: Message) => {
			if (event.type === 'parameterUpdate') {
				setParameters((prevState) => {
					return {
						...prevState,
						[event.data.parameterId]: event.data.value,
					};
				});
			}
		};
		const unsubscribe = messageBus.subscribe(handlePluginMessage);
		return () => {
			unsubscribe();
		};
	}, [messageBus]);

	return (
		<MessageBusContext.Provider value={messageBus}>
			<GlobalParametersContext.Provider
				value={{ parameters, setParameters }}
			>
				<div className='flex-col h-[100vh] w-[100vw] px-1'>
					<TopBar />
					<IrLoader />
					<div className='flex gap-1 py-1 h-[60vh]'>
						<div className='w-[60%] h-full flex secondary rounded-sm p-1 gap-1 text-center'>
							<LowpassControls />
							<BellControls />
							<HighpassControls />
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
