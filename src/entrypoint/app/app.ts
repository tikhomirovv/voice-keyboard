import App from "@/entrypoint/app/App.vue";
import appRouter from "@/router/app";
import { shortcutService } from "@/services/shortcuts/shortcutsService";
import WindowManager from "@/lib/system/windowManager";
import { WINDOW as CONTROLS_WINDOW } from "@/lib/windows/controls";
import { createApp } from "vue";
import Logger from "@/lib/system/logger";

export function init() {
  const app = createApp(App);
  app.use(appRouter);
  app.mount("#app");
  WindowManager.destroyWindow(CONTROLS_WINDOW).then(() => {
    WindowManager.initWindowOnce(CONTROLS_WINDOW);
  });

  shortcutService.init().catch((error) => {
    Logger.error("Ошибка при инициализации горячих клавиш:", error);
  });
  window.addEventListener("beforeunload", () => {
    shortcutService.cleanup().catch((error) => {
      Logger.error("Ошибка при очистке горячих клавиш:", error);
    });
  });
}
