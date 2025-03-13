import { createApp } from "vue";
import Controls from "@/entrypoint/controls/Controls.vue";
import controlsRouter from "@/router/controls";

import { audioEventService } from "./services";
export function init() {
  const app = createApp(Controls);
  app.provide("audioEventsService", audioEventService);
  app.use(controlsRouter);
  app.mount("#app");
}
