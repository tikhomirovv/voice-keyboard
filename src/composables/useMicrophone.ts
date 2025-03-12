import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Microphone } from "@/types/microphone";
import {
  get as getFromStorage,
  save as saveToStorage,
} from "@/services/microphone/microphoneStorage";
import Logger from "@/lib/system/logger";

export function useMicrophone() {
  const refresh = async () => {
    return getMicrophones();
  };
  const microphones = ref<Microphone[]>([]);
  const selected = ref<string | null>(null);

  const getMicrophones = async () => {
    try {
      const result = await invoke<string>("get_microphones"); // Получаем JSON-строку с микрофонами
      microphones.value = JSON.parse(result) as Microphone[]; // Парсим JSON в массив объектов Microphone
      if (microphones.value.length <= 0) {
        throw Error("Нет ни одного микрофона");
      }
      const savedMicrophone = await getFromStorage(); // Получаем сохраненный микрофон из хранилища
      // Проверяем, есть ли сохраненный микрофон в списке микрофонов
      const foundMicrophone = microphones.value.find(
        (mic) => mic.id === savedMicrophone.id
      );
      if (foundMicrophone) {
        selected.value = foundMicrophone.id; // Если найден, выбираем его
      } else {
        selected.value = microphones.value[0].id; // Если не найден, выбираем первый из списка
      }
    } catch (error) {
      Logger.error("Ошибка загрузки микрофонов:", error);
      // Если возникла ошибка, выбрасываем ее, чтобы она могла быть обработана на более высоком уровне
      throw error;
    }
  };

  const set = async (id: string) => {
    try {
      // Attempt to save the new selected microphone to storage
      await saveMicrophoneToStorage(id);
      selected.value = id;
      Logger.info(`Selected microphone saved to storage: ${id}`);
    } catch (error) {
      Logger.error("Failed to save selected microphone to storage:", error);
    }
  };

  // Function to save the microphone to storage
  const saveMicrophoneToStorage = async (microphoneId: string) => {
    const microphone = microphones.value.find((mic) => mic.id === microphoneId);
    if (microphone) {
      // Assuming there's a function to save microphone settings to storage
      await saveToStorage(microphone);
    } else {
      Logger.error(`Microphone with ID ${microphoneId} not found.`);
    }
  };

  refresh();

  return {
    refresh,
    selected,
    set,
    microphones,
  };
}
