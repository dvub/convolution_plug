import { IrData } from "@/bindings/IrData";
import { Message } from "@/bindings/Message";
import { MessageBusContext } from "@/contexts/MessageBusContext";
import { sendToPlugin } from "@/lib";
import { UploadIcon } from "@radix-ui/react-icons";
import { parseBuffer } from "music-metadata";

import {
  ChangeEvent,
  Dispatch,
  SetStateAction,
  useContext,
  useEffect,
  useRef,
  useState,
} from "react";
import WaveSurfer from "wavesurfer.js";

export function IrLoader() {
  const waveSurferRef = useRef<WaveSurfer | null>(null);
  const containerRef = useRef(null);

  const messageBus = useContext(MessageBusContext)!;
  const [fileInfo, setFileInfo] = useState<IrData | undefined>(undefined);

  useEffect(() => {
    const waveSurfer = WaveSurfer.create({
      container: containerRef.current!,
      height: "auto",
      waveColor: "#ecf0ef",
      normalize: true,
      cursorWidth: 0,
      interact: false,
    });

    waveSurfer.on("init", () => {
      waveSurferRef.current = waveSurfer;
    });

    const handlePluginMessage = (event: Message) => {
      if (event.type !== "initResponse") {
        return;
      }

      const irData = event.data.irData;
      if (!irData) {
        return;
      }

      const blob = new Blob([new Uint8Array(irData.rawBytes)], {
        type: "wav",
      });
      waveSurfer.loadBlob(blob);
      setFileInfo(irData);
    };

    const unsubscribe = messageBus.subscribe(handlePluginMessage);
    return () => {
      unsubscribe();
      waveSurfer.destroy();
    };
  }, [messageBus]);

  function onFileChange(event: ChangeEvent<HTMLInputElement>) {
    if (!event.target.files || event.target.files.length === 0) {
      return;
    }
    const fileName = event.target.files[0].name;

    // TODO: fix nested code
    const reader = new FileReader();
    reader.onload = () => {
      const arrayBuffer = reader.result as ArrayBuffer;
      const bytes = new Uint8Array(arrayBuffer);
      parseBuffer(bytes).then((x) => {
        const info = x.format;

        const irData: IrData = {
          name: fileName,
          rawBytes: [...bytes],
          // TODO: handle if these are undefined!
          lengthSeconds: info.duration!,
          numChannels: info.numberOfChannels!,
          sampleRate: info.sampleRate!,
        };

        setFileInfo(irData);

        sendToPlugin({
          type: "irUpdate",
          data: irData,
        });
      });
    };
    // now pass the file into the reader

    // we can also be sure that there will be a file here (i think)
    const input = event.target.files[0];
    reader.readAsArrayBuffer(input);

    // finally, visualization
    waveSurferRef.current?.loadBlob(input);
  }

  const IrInfoDisplay = (
    <div>
      <h1 className="text-sm">
        {/* https://stackoverflow.com/questions/1199352/smart-way-to-truncate-long-strings*/}
        {fileInfo?.name.replace(/(.{20})..+/, "$1…")}
      </h1>
      <h1 className="text-xs">Len: {fileInfo?.lengthSeconds.toFixed(3)}s</h1>
      <h1 className="text-xs">Channels: {fileInfo?.numChannels}</h1>
      <h1 className="text-xs">SR: {fileInfo?.sampleRate} Hz</h1>
    </div>
  );
  const defaultIrDisplay = <h1>No IR Loaded.</h1>;

  return (
    <div className="w-full secondary rounded-sm h-[35vh]">
      <div className="h-[35vh] p-1 flex gap-1">
        <div className="w-[50%] h-full rounded-sm">
          <div ref={containerRef} className="h-full" />
        </div>
        <div className="w-[50%] flex flex-col gap-1">
          <div className="h-[50%] bg-zinc-500 rounded-sm p-1">
            {fileInfo ? IrInfoDisplay : defaultIrDisplay}
          </div>
          <div className="w-full h-[50%] bg-zinc-500 rounded-sm p-1 text-sm flex flex-col justify-between">
            <div className="flex w-full justify-between bg-black">
              <h1>Normalize</h1>
              <button>Off</button>
              <h1>db</h1>
            </div>
            <div className="flex w-full justify-between bg-black">
              <h1>Resample</h1>
              <button>Off</button>
              <h1>Current SR</h1>
            </div>

            <div className="flex gap-5 items-center">
              <input
                id="inp"
                type="file"
                onChange={onFileChange}
                className="hidden"
              />
              <label htmlFor="inp" className="hover:cursor-pointer">
                <UploadIcon />
              </label>
              <h1>Refresh</h1>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
