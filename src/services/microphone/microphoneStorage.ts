import { load } from "@tauri-apps/plugin-store";
import type { MicrophoneConfig } from "@/types/microphone";
import { DEFAULT_MICROPHONE } from "@/lib/microphone";
import Logger from "@/lib/system/logger";

const store = await load("microphone.json", { autoSave: false });
const KEY = "microphone";

// Получаем текущие настройки, если их нет - используем дефолтные
export async function get(): Promise<MicrophoneConfig> {
  try {
    const shortcuts = await store.get(KEY);
    return (shortcuts as MicrophoneConfig) ?? { ...DEFAULT_MICROPHONE };
  } catch (error) {
    Logger.error("Ошибка при загрузке настроек горячих клавиш:", error);
    return { ...DEFAULT_MICROPHONE };
  }
}

// Сохраняем измененные настройки
export async function save(config: MicrophoneConfig): Promise<void> {
  try {
    await store.set(KEY, config);
    await store.save();
  } catch (error) {
    Logger.error("Ошибка при сохранении настроек микрофона:", error);
    throw error;
  }
}

// Сбрасываем к дефолтным значениям
export async function reset(): Promise<MicrophoneConfig> {
  const defaultConfig = { ...DEFAULT_MICROPHONE };
  await save(defaultConfig);
  return defaultConfig;
}
