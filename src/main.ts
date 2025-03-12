import { createApp } from "vue";
import App from "@/App.vue";
import Controls from "@/Controls.vue";
import appRouter from "@/router/app";
import controlsRouter from "@/router/controls";
import "@/assets/main.css";
import { shortcutService } from "@/services/shortcuts/shortcutsService";
import { getCurrentWindow } from "@tauri-apps/api/window";
import Logger from "@/lib/system/logger";
import WindowManager from "@/lib/system/windowManager";
import { WINDOW as CONTROLS_WINDOW } from "@/lib/windows/controls";

const currentWindow = getCurrentWindow();
switch (currentWindow.label) {
  case "main":
    initMainWindow();
    break;
  case "controls":
    initControlsWindow();
    break;
}

function initMainWindow() {
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

function initControlsWindow() {
  const app = createApp(Controls);
  app.use(controlsRouter);
  app.mount("#app");
}
