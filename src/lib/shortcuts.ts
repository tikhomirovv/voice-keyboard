import type { ShortcutConfig } from "@/types/shortcuts";
import { useTranscribe } from "@/composables/useTranscribe";

const { start, stop } = useTranscribe();

export const DEFAULT_SHORTCUTS: ShortcutConfig = {
  toggleRecording: {
    id: "toggleRecording",
    name: "Включить/выключить запись",
    description: "Включает или выключает запись голоса",
    key: "CommandOrControl+Shift+Alt+R",
    handlers: {
      onPressed: () => {
        start();
        // Здесь будет логика начала записи
        console.log("Начало записи");
      },
      onReleased: () => {
        stop();
        // Здесь будет логика окончания записи
        console.log("Конец записи");
      },
    },
  },
  toggleVisibility: {
    id: "toggleVisibility",
    name: "Показать/скрыть окно",
    description: "Показывает или скрывает главное окно приложения",
    key: "CommandOrControl+Shift+Alt+D",
    handlers: {
      onPressed: () => {
        // Здесь будет логика переключения видимости окна
        console.log("Переключение видимости окна");
      },
    },
  },
};
