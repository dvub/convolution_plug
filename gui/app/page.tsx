"use client";

import { useEffect } from "react";

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

import { IPC } from "./thing";
export default function Home() {
  useEffect(() => {
    IPC.send("init");

    IPC.on((message) => {
      console.log("Receiving message:", message);
    });
  }, []);

  return <h1>hi</h1>;
}
