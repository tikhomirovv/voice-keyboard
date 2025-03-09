import { load } from "@tauri-apps/plugin-store";
import type { ShortcutConfig } from "@/types/shortcuts";
import { DEFAULT_SHORTCUTS } from "@/lib/shortcuts";

const store = await load("shortcuts.json", { autoSave: false });
const SHORTCUTS_KEY = "shortcuts";

// Получаем текущие настройки, если их нет - используем дефолтные
export async function getShortcuts(): Promise<ShortcutConfig> {
  try {
    const shortcuts = await store.get(SHORTCUTS_KEY);
    return (shortcuts as ShortcutConfig) ?? { ...DEFAULT_SHORTCUTS };
  } catch (error) {
    console.error("Ошибка при загрузке настроек горячих клавиш:", error);
    return { ...DEFAULT_SHORTCUTS };
  }
}

// Сохраняем измененные настройки
export async function saveShortcuts(shortcuts: ShortcutConfig): Promise<void> {
  try {
    await store.set(SHORTCUTS_KEY, shortcuts);
    await store.save();
  } catch (error) {
    console.error("Ошибка при сохранении настроек горячих клавиш:", error);
    throw error;
  }
}

// Сбрасываем к дефолтным значениям
export async function resetToDefault(): Promise<ShortcutConfig> {
  const defaultConfig = { ...DEFAULT_SHORTCUTS };
  await saveShortcuts(defaultConfig);
  return defaultConfig;
}
