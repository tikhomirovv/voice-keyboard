import { invoke } from "@tauri-apps/api/core";
import { useMicrophone } from "@/composables/useMicrophone";
import { createRecordEventChannel } from "@/lib/events";
export function useTranscribe() {
  const { selected, refresh } = useMicrophone();
  const stop = async () => {
    await invoke<string>("stop_record");
    await invoke<string>("start_transcribation");
  };
  const start = async () => {
    await refresh(); // микрофона и выбираем тот, который сохранен в хранилище
    console.log("Start record", selected.value);
    if (!selected.value) {
      alert("Выберите микрофон");
      return;
    }

    invoke("start_record", {
      deviceId: selected.value,
      onEvent: createRecordEventChannel(),
    });
  };

  return {
    start,
    stop,
  };
}
