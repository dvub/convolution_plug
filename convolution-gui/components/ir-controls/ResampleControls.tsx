import { sendToPlugin } from "@/lib";

export function ResampleControls() {
  function handleUpdate() {
    sendToPlugin({
      type: "irConfigUpdate",
      data: {
        normalizeIrs: false,
        resample: true,
        normalizationLevel: 0,
      },
    });
  }

  return (
    <div className="flex w-full justify-between bg-zinc-500 p-1 rounded-sm">
      <h1 className="w-[33%]">Resample:</h1>
      <button className="w-[33%]">Off</button>
      <button className="w-[33%]" onClick={handleUpdate}>
        In. SR
      </button>
    </div>
  );
}
