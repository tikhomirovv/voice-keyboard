import { register, unregister } from "@tauri-apps/plugin-global-shortcut";
import { getShortcuts, saveShortcuts } from "./shortcutsStorage";
import { Shortcut, KeyState } from "@/types/shortcuts";

class ShortcutService {
  private registeredShortcuts: { [key: string]: Shortcut } = {};
  private initialized: boolean = false;

  /**
   * Checks if a specific string is present in the 'key' property of any registered shortcut.
   * The 'key' property can be either a string or an array of strings.
   *
   * @param key The string to search for in the 'key' property of registered shortcuts.
   * @returns Returns true if the targetString is found in any registered shortcut's 'key' property, false otherwise.
   */
  private checkShortcutKeyPresence(key: string): boolean {
    return Object.values(this.registeredShortcuts).some((shortcut) => {
      const keys = Array.isArray(shortcut.key) ? shortcut.key : [shortcut.key];
      return keys.includes(key);
    });
  }

  private async registerShortcut(shortcut: Shortcut) {
    const keys = Array.isArray(shortcut.key) ? shortcut.key : [shortcut.key];
    if (keys.some((key) => this.checkShortcutKeyPresence(key))) {
      console.warn(`Горячая клавиша ${shortcut.id} уже зарегистрирована`);
      return;
    }

    try {
      await register(shortcut.key, (event) => {
        const state: String = event.state;
        console.log(`Shortcut ${shortcut.name} ${event.state}`);

        // Вызываем соответствующий обработчик
        if (state === KeyState.Pressed && shortcut.handlers?.onPressed) {
          shortcut.handlers.onPressed();
        } else if (
          state === KeyState.Released &&
          shortcut.handlers?.onReleased
        ) {
          shortcut.handlers.onReleased();
        }
      });
      this.registeredShortcuts[shortcut.id] = shortcut;
    } catch (error) {
      console.error(
        `Ошибка при регистрации горячей клавиши ${shortcut.name}:`,
        error
      );
      throw error; // Пробрасываем ошибку дальше для обработки
    }
  }

  private async unregisterShortcut(key: string) {
    if (!this.checkShortcutKeyPresence(key)) {
      return; // Если клавиша не зарегистрирована, ничего не делаем
    }

    try {
      await unregister(key);
      delete this.registeredShortcuts[key];
    } catch (error) {
      console.error(
        `Ошибка при отмене регистрации горячей клавиши ${key}:`,
        error
      );
      throw error;
    }
  }

  async init() {
    // Проверяем, не инициализирован ли уже сервис
    if (this.initialized) {
      console.warn("ShortcutService уже инициализирован");
      return;
    }

    try {
      const shortcuts = await getShortcuts();
      for (const shortcut of Object.values(shortcuts)) {
        await this.registerShortcut(shortcut);
      }
      this.initialized = true;
    } catch (error) {
      console.error("Ошибка при инициализации горячих клавиш:", error);
      throw error;
    }
  }

  async updateShortcut(id: string, newKey: string) {
    try {
      const shortcuts = await getShortcuts();
      const shortcut = shortcuts[id];

      if (!shortcut) {
        throw new Error(`Горячая клавиша с id ${id} не найдена`);
      }

      // Проверяем, не используется ли уже новая комбинация
      if (this.checkShortcutKeyPresence(newKey) && newKey !== shortcut.key) {
        throw new Error(`Комбинация ${newKey} уже используется`);
      }

      // Отменяем регистрацию старого сочетания
      await this.unregisterShortcut(shortcut.key);

      // Обновляем сочетание
      shortcut.key = newKey;
      await saveShortcuts(shortcuts);

      // Регистрируем новое сочетание
      await this.registerShortcut(shortcut);
    } catch (error) {
      console.error(`Ошибка при обновлении горячей клавиши:`, error);
      throw error;
    }
  }

  async cleanup() {
    try {
      for (const shortcutKey in this.registeredShortcuts) {
        await this.unregisterShortcut(
          this.registeredShortcuts[shortcutKey].key
        );
      }
      this.initialized = false;
    } catch (error) {
      console.error("Ошибка при очистке горячих клавиш:", error);
      throw error;
    }
  }

  // Геттер для проверки статуса инициализации
  isInitialized(): boolean {
    return this.initialized;
  }
}

// Создаем единственный экземпляр сервиса
export const shortcutService = new ShortcutService();
