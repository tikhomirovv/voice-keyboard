import { createApp } from "vue";
import App from "@/App.vue";
import router from "@/router";
import "@/assets/main.css";
import "@/lib/windows/controls";
import { shortcutService } from "@/services/shortcuts/shortcutsService";
import { getCurrentWindow } from "@tauri-apps/api/window";

const currentWindow = getCurrentWindow();
console.log("currentWindow", currentWindow);
if (currentWindow.label === "controls") {
  shortcutService.init().catch((error) => {
    console.error("Ошибка при инициализации горячих клавиш:", error);
  });
  // Добавляем обработчик для очистки при закрытии приложения
  window.addEventListener("beforeunload", () => {
    shortcutService.cleanup().catch((error) => {
      console.error("Ошибка при очистке горячих клавиш:", error);
    });
  });
}

const app = createApp(App);
app.use(router);
app.mount("#app");
