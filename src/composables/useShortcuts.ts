import { ref, onMounted } from "vue";
import { getShortcuts } from "@/services/shortcuts/shortcutsStorage";
import { shortcutService } from "@/services/shortcuts/shortcutsService";
import type { ShortcutConfig } from "@/types/shortcuts";
import { useMagicKeys, watchDeep } from "@vueuse/core";
import Logger from "@/lib/system/logger";

export function useShortcuts() {
  const list = ref<ShortcutConfig | null>(null);
  const isLoading = ref(false);
  const error = ref<string | null>(null);
  const isPressing = ref(false);
  const { current: currentPressed } = useMagicKeys();
  const lastPressedKeys = ref<Set<string>>(new Set());

  const updateLastPressed = () => {
    const currentLength = currentPressed.size;
    if (currentLength > 4) return;
    let pressedLength = lastPressedKeys.value.size;
    if (!isPressing.value) {
      lastPressedKeys.value.clear();
      pressedLength = 0;
    }
    isPressing.value = currentLength > 0;
    if (currentLength >= pressedLength) {
      currentPressed.forEach((key) => {
        if (!lastPressedKeys.value.has(key)) {
          lastPressedKeys.value.add(key);
        }
      });
    }
  };

  const loadShortcuts = async () => {
    try {
      isLoading.value = true;
      error.value = null;
      list.value = await getShortcuts();
    } catch (err) {
      error.value = "Ошибка при загрузке настроек горячих клавиш";
      Logger.error(error.value, err);
    } finally {
      isLoading.value = false;
    }
  };

  const updateShortcut = async (id: string, newKey: string) => {
    try {
      isLoading.value = true;
      error.value = null;
      await shortcutService.updateShortcut(id, newKey);
      await loadShortcuts();
    } catch (err) {
      error.value = "Ошибка при обновлении горячей клавиши";
      Logger.error(error.value, err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  };

  const deleteShortcut = async (id: string) => {
    try {
      isLoading.value = true;
      error.value = null;
      await shortcutService.deleteShortcut(id);
      await loadShortcuts();
    } catch (err) {
      error.value = "Ошибка при удалении горячей клавиши";
      Logger.error(error.value, err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  };

  // Загружаем начальное состояние при монтировании компонента
  onMounted(() => {
    loadShortcuts();
  });
  watchDeep(currentPressed, updateLastPressed);

  return {
    list,
    isLoading,
    error,
    refresh: loadShortcuts,
    updateShortcut,
    deleteShortcut,
    lastPressedKeys,
    isPressing,
  };
}
