"use client";

import { MessageBus, MessageBusContext } from "@/contexts/MessageBusContext";
import { useEventDispatcher } from "@/hooks/useEventDispatcher";
import { useEffect, useState } from "react";

import {
  GlobalParameters,
  GlobalParametersContext,
} from "@/contexts/GlobalParamsContext";

import { Message } from "@/bindings/Message";

import { sendToPlugin } from "@/lib";
import { Knob } from "@/components/knobs/Knob";
import { dbToGain, gainToDb, NormalisableRange } from "@/lib/utils";

export default function Home() {
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
    sendToPlugin({ type: "init" });

    const handlePluginMessage = (event: Message) => {
      console.log(event);
      switch (event.type) {
        case "parameterUpdate":
          setParameters((prevState) => {
            return {
              ...prevState,
              [event.data.parameterId]: JSON.parse(event.data.value),
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

  return (
    <MessageBusContext.Provider value={messageBus}>
      <GlobalParametersContext.Provider value={{ parameters, setParameters }}>
        <button
          onClick={() =>
            sendToPlugin({
              type: "parameterUpdate",
              data: {
                parameterId: "highpass_enabled",
                value: "true",
              },
            })
          }
        >
          BUTTON
        </button>
        <Knob
          minValue={0}
          maxValue={1}
          defaultValue={dbToGain(0)}
          label={""}
          size={50}
          range={new NormalisableRange(0, 1, 0.5)}
          parameter="gain"
          valueRawDisplayFn={(x) => `${gainToDb(x).toFixed(2)} dB`}
        ></Knob>
      </GlobalParametersContext.Provider>
    </MessageBusContext.Provider>
  );
}
