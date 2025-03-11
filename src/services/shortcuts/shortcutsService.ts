import { register, unregister } from "@tauri-apps/plugin-global-shortcut";
import { getShortcuts, saveShortcuts } from "./shortcutsStorage";
import { Shortcut, KeyState } from "@/types/shortcuts";
import { DEFAULT_SHORTCUTS } from "@/lib/shortcuts";

class ShortcutService {
  private registeredShortcuts: { [key: string]: Shortcut } = {};
  private initialized: boolean = false;
  private isCleaningUp: boolean = false;

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
    // Если идет процесс очистки, не регистрируем новые клавиши
    if (this.isCleaningUp) {
      console.warn("Регистрация горячих клавиш невозможна во время очистки");
      return;
    }

    const keys = Array.isArray(shortcut.key) ? shortcut.key : [shortcut.key];

    // Проверяем, не зарегистрирована ли уже эта комбинация
    if (keys.some((key) => this.checkShortcutKeyPresence(key))) {
      // Если клавиша уже зарегистрирована, сначала отменяем регистрацию
      try {
        await this.unregisterShortcut(shortcut.key);
      } catch (error) {
        console.warn(
          `Не удалось отменить регистрацию клавиши ${shortcut.key}:`,
          error
        );
      }
    }

    try {
      await register(shortcut.key, (event) => {
        const state: String = event.state;
        console.log(
          `Shortcut ${shortcut.name} ${event.state}`,
          shortcut.handlers
        );

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
      throw error;
    }
  }

  private async unregisterShortcut(key: string) {
    if (!this.checkShortcutKeyPresence(key)) {
      return;
    }

    try {
      await unregister(key);
      // Находим и удаляем все шорткаты с этим ключом
      Object.keys(this.registeredShortcuts).forEach((id) => {
        if (this.registeredShortcuts[id].key === key) {
          delete this.registeredShortcuts[id];
        }
      });
    } catch (error) {
      console.error(
        `Ошибка при отмене регистрации горячей клавиши ${key}:`,
        error
      );
      throw error;
    }
  }

  async init() {
    // Если сервис уже инициализирован, сначала очищаем
    if (this.initialized) {
      console.warn("ShortcutService уже инициализирован, выполняем очистку");
      await this.cleanup();
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

  async deleteShortcut(id: string) {
    return await this.updateShortcut(id, DEFAULT_SHORTCUTS[id].key);
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
      await this.registerShortcut(shortcut);
      await saveShortcuts(shortcuts);
    } catch (error) {
      console.error(`Ошибка при обновлении горячей клавиши:`, error);
      throw error;
    }
  }

  async cleanup() {
    if (this.isCleaningUp) {
      console.warn("Очистка уже выполняется");
      return;
    }

    this.isCleaningUp = true;
    try {
      const shortcuts = { ...this.registeredShortcuts };
      for (const shortcut of Object.values(shortcuts)) {
        await this.unregisterShortcut(shortcut.key);
      }
      this.registeredShortcuts = {};
      this.initialized = false;
    } catch (error) {
      console.error("Ошибка при очистке горячих клавиш:", error);
      throw error;
    } finally {
      this.isCleaningUp = false;
    }
  }

  // Геттер для проверки статуса инициализации
  isInitialized(): boolean {
    return this.initialized;
  }
}

// Создаем единственный экземпляр сервиса
export const shortcutService = new ShortcutService();
