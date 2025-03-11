// Тип состояния клавиши
export enum KeyState {
  Pressed = "Pressed",
  Released = "Released",
}

// Интерфейс для обработчиков событий
export interface ShortcutHandlers {
  onPressed?: () => void;
  onReleased?: () => void;
}

export interface Shortcut {
  id: string;
  name: string;
  description: string;
  key: string;
  handlers?: ShortcutHandlers;
}

export interface ShortcutConfig {
  [key: string]: Shortcut;
}

// Тип для хранения в store только необходимых данных
export interface StoredShortcut {
  id: string;
  key: string;
}

export interface StoredShortcutConfig {
  [key: string]: StoredShortcut;
}
