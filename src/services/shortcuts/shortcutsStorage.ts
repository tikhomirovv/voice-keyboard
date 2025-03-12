import { load } from "@tauri-apps/plugin-store";
import type { ShortcutConfig, StoredShortcutConfig } from "@/types/shortcuts";
import { DEFAULT_SHORTCUTS } from "@/lib/shortcuts";
import Logger from "@/lib/system/logger";

const store = await load("shortcuts.json", { autoSave: false });
const SHORTCUTS_KEY = "shortcuts";

// Преобразует полную конфигурацию в формат для хранения
function toStorageFormat(shortcuts: ShortcutConfig): StoredShortcutConfig {
  const storage: StoredShortcutConfig = {};
  Object.entries(shortcuts).forEach(([id, shortcut]) => {
    storage[id] = {
      id: shortcut.id,
      key: shortcut.key,
    };
  });
  return storage;
}

// Получаем текущие настройки, если их нет - используем дефолтные
export async function getShortcuts(): Promise<ShortcutConfig> {
  try {
    const result = { ...DEFAULT_SHORTCUTS };
    const storedShortcuts = (await store.get(
      SHORTCUTS_KEY
    )) as StoredShortcutConfig;

    // Обновляем только ключи из хранилища
    if (storedShortcuts) {
      Object.entries(storedShortcuts).forEach(([id, stored]) => {
        if (result[id]) {
          result[id] = {
            ...result[id],
            ...stored,
          };
        }
      });
    }

    return result;
  } catch (error) {
    Logger.error("Ошибка при загрузке настроек горячих клавиш:", error);
    return { ...DEFAULT_SHORTCUTS };
  }
}

// Сохраняем измененные настройки
export async function saveShortcuts(shortcuts: ShortcutConfig): Promise<void> {
  try {
    const storageFormat = toStorageFormat(shortcuts);
    await store.set(SHORTCUTS_KEY, storageFormat);
    await store.save();
  } catch (error) {
    Logger.error("Ошибка при сохранении настроек горячих клавиш:", error);
    throw error;
  }
}

// Сбрасываем к дефолтным значениям
export async function resetToDefault(): Promise<ShortcutConfig> {
  const defaultConfig = { ...DEFAULT_SHORTCUTS };
  await saveShortcuts(defaultConfig);
  return defaultConfig;
}
