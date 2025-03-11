import type { ShortcutConfig } from "@/types/shortcuts";
import { useTranscribe } from "@/composables/useTranscribe";
import { type } from "@tauri-apps/plugin-os";
import type { OsType } from "@tauri-apps/plugin-os";

const { start, stop } = useTranscribe();
const keyMap: { [key: string]: string } = {
  commandorcontrol: type() === ("macos" as OsType) ? "command" : "control",
  meta: "super",
};
export function normalizeKey(key: string): string {
  // Convert the key to lowercase for case-insensitive comparison
  let lowerCaseKey = key.toLowerCase();
  // Log the original key for debugging purposes
  console.log(`Original key: ${key}`);
  // Iterate through the keyMap to replace keys with their corresponding values
  Object.keys(keyMap).forEach((mapKey) => {
    // Replace the mapKey with its corresponding value in keyMap
    lowerCaseKey = lowerCaseKey.replace(mapKey, keyMap[mapKey]);
    // Log the replacement for debugging purposes
    console.log(`Replacing ${mapKey} with ${keyMap[mapKey]}`);
  });
  // Return the normalized key
  return lowerCaseKey;
}

export const DEFAULT_SHORTCUTS: ShortcutConfig = {
  toggleRecording: {
    id: "toggleRecording",
    name: "Включить/выключить запись",
    description: "Включает или выключает запись голоса",
    key: normalizeKey("commandorcontrol+shift+alt+r"),
    handlers: {
      onPressed: () => {
        start();
      },
      onReleased: () => {
        stop();
      },
    },
  },
  toggleVisibility: {
    id: "toggleVisibility",
    name: "Показать/скрыть окно",
    description: "Показывает или скрывает главное окно приложения",
    key: normalizeKey("commandorcontrol+shift+alt+d"),
    handlers: {
      onPressed: () => {
        // Здесь будет логика переключения видимости окна
        console.log("Переключение видимости окна");
      },
    },
  },
};
