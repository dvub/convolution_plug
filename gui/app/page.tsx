'use client';

import { MessageBus, MessageBusContext } from '@/contexts/MessageBusContext';
import { useMessageDispatcher } from '@/hooks/useMessageDispatcher';
import { useEffect, useState } from 'react';

import { sendToPlugin } from '@/lib';
import { IRManager } from '@/components/ir-controls/IRManager';
import LowpassControls from '@/components/filter-controls/LowpassControls';
import BellControls from '@/components/filter-controls/BellControls';
import HighpassControls from '@/components/filter-controls/HighpassControls';
import TopBar from '@/components/TopBar';
import GainControls from '@/components/GainControls';
// import { Message } from "@/bindings/Message";
import { useMessageSubscriber } from '@/hooks/useMessageSubscriber';

/*
export default function Home() {
  const [messageBus] = useState(new MessageBus());

  // TODO: possibly change/fix loading behavior?
  // right now this works by simply making the entire page have 0 opacity
  // if we do something conventional, such as a placeholder loading element
  // it prevents elements (e.g. knobs) from being loaded and receiving their initial values
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    sendToPlugin({ type: "init" });
  }, []);

  useMessageDispatcher(messageBus);
  useMessageSubscriber((event: Message) => {
    console.log(event);
    if (event.type === "initResponse") {
      setIsLoading(false);
    }
  }, messageBus);

  return (
    <MessageBusContext.Provider value={messageBus}>
      <div style={{ opacity: isLoading ? 0 : 1 }}>
        <TopBar />
        <IRManager />
        <div className="flex gap-1 py-1 h-[60vh]">
          <div className="w-[60%] flex secondary rounded-sm p-1 gap-1">
            <HighpassControls />
            <BellControls />
            <LowpassControls />
          </div>
          <GainControls />
        </div>
      </div>
    </MessageBusContext.Provider>
  );
}
*/

import { IPC } from './thing';
export default function Home() {
	useEffect(() => {
		console.log('hello!');
		IPC.on('message', (m) => {
			console.log(m);
		});
	}, []);

	return <h1>hi</h1>;
}
