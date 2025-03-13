import App from "@/entrypoint/app/App.vue";
import appRouter from "@/router/app";
import { shortcutService } from "@/services/shortcuts/shortcutsService";
import WindowManager from "@/lib/system/windowManager";
import { WINDOW as CONTROLS_WINDOW } from "@/lib/windows/controls";
import { createApp } from "vue";
import Logger from "@/lib/system/logger";

import { getCurrentWindow } from "@tauri-apps/api/window";

export async function init() {
  const app = createApp(App);
  app.use(appRouter);
  app.mount("#app");

  // Create controls window
  WindowManager.destroyWindow(CONTROLS_WINDOW).then(() => {
    WindowManager.initWindowOnce(CONTROLS_WINDOW);
  });

  // Init shortcuts
  shortcutService.init().catch((error) => {
    Logger.error("Ошибка при инициализации горячих клавиш:", error);
  });

  // On close and destroy events
  const mainWindow = getCurrentWindow();
  await mainWindow.listen("tauri://close-requested", () => {
    WindowManager.destroyWindow(CONTROLS_WINDOW);
    shortcutService.cleanup();
    Logger.debug("App main window destroyed");
    mainWindow.destroy();
  });

  // window.addEventListener("beforeunload", () => {
  //   shortcutService.cleanup().catch((error) => {
  //     Logger.error("Ошибка при очистке горячих клавиш:", error);
  //   });
  // });
}
