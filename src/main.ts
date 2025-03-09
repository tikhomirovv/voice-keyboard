import { createApp } from "vue";
import App from "@/App.vue";
import router from "@/router";
import "@/assets/main.css";
import { shortcutService } from "@/services/shortcuts/shortcutsService";

// Инициализируем горячие клавиши
shortcutService.init().catch((error) => {
  console.error("Ошибка при инициализации горячих клавиш:", error);
});

const app = createApp(App);
app.use(router);
app.mount("#app");
