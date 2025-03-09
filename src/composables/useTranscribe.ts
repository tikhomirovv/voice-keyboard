import { invoke } from "@tauri-apps/api/core";
import { useMicrophone } from "@/composables/useMicrophone";

export function useTranscribe() {
  const { selected, refresh } = useMicrophone();

  const stop = async () => {
    await invoke<string>("stop_record");
  };
  const start = async () => {
    await refresh(); // микрофона и выбираем тот, который сохранен в хранилище
    console.log("Start record", selected.value);
    if (!selected.value) {
      alert("Выберите микрофон");
      return;
    }

    await invoke("start_record", {
      deviceId: selected.value,
    });
  };

  return {
    start,
    stop,
  };
}
