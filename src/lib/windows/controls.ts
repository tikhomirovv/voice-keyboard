import Logger from "@/lib/system/logger";
// import { LogicalPosition } from "@tauri-apps/api/dpi";
import { Window } from "@/types/window";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

export const WINDOW: Window = {
  label: "controls",
  options: {
    title: "controls",
    url: "/controls",
    center: true,
    minWidth: 20,
    width: 300,
    maxWidth: 100,
    height: 30,
    minHeight: 30,
    maxHeight: 60,
    maximizable: false,
    // closable: false,
    closable: true,
    resizable: false,
    minimizable: false,
    transparent: true,
    decorations: false,
    // decorations: true,
    alwaysOnTop: true,
    // shadow: false, // убирает тень (границы)
    shadow: true,
    skipTaskbar: true,
  },
  onCreated: async (_: WebviewWindow) => {
    // await ww.setPosition(new LogicalPosition(1600, 840));
  },
  onError: async (_: WebviewWindow, error: any) => {
    Logger.error("an error happened creating the webview", error);
  },
  onDestroyed: async (w: WebviewWindow) => {
    Logger.debug("Controls window onDestroyed");
    w.destroy();
  },
};
