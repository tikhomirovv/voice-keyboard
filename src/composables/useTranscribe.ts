import { invoke } from "@tauri-apps/api/core";
import { useMicrophone } from "@/composables/useMicrophone";
// import Logger from "@/lib/system/logger";

export function useTranscribe() {
  const { selected, refresh } = useMicrophone();
  const stop = async () => {
    await invoke<string>("stop_record");
    // await invoke<string>("start_transcribation");
  };
  const start = async () => {
    await refresh();
    if (!selected.value) {
      alert("Выберите микрофон");
      return;
    }

    invoke("start_record", {
      deviceId: selected.value,
    });
  };

  return {
    start,
    stop,
  };
}
