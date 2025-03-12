import type { ShortcutConfig } from "@/types/shortcuts";
import { useTranscribe } from "@/composables/useTranscribe";
import { type } from "@tauri-apps/plugin-os";
import type { OsType } from "@tauri-apps/plugin-os";
import Logger from "@/lib/system/logger";

const { start, stop } = useTranscribe();
const keyMap: { [key: string]: string } = {
  commandorcontrol: type() === ("macos" as OsType) ? "command" : "control",
  meta: "super",
};
export function normalizeKey(key: string): string {
  // Convert the key to lowercase for case-insensitive comparison
  let lowerCaseKey = key.toLowerCase();
  // Iterate through the keyMap to replace keys with their corresponding values
  Object.keys(keyMap).forEach((mapKey) => {
    // Replace the mapKey with its corresponding value in keyMap
    lowerCaseKey = lowerCaseKey.replace(mapKey, keyMap[mapKey]);
  });
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
        Logger.info("Переключение видимости окна");
      },
    },
  },
};
