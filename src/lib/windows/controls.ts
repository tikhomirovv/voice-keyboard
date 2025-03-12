import Logger from "@/lib/system/logger";
// import { LogicalPosition } from "@tauri-apps/api/dpi";
import { Window } from "@/types/window";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

export const WINDOW: Window = {
  label: "controls",
  options: {
    title: "controls",
    url: "/controls",
    width: 200,
    maxWidth: 200,
    height: 100,
    center: true,
    maxHeight: 100,
    maximizable: false,
    closable: false,
    resizable: false,
    minimizable: false,
    transparent: true,
    decorations: false,
    alwaysOnTop: true,
    shadow: true, // убирает тень (границы)
    skipTaskbar: true,
  },
  onCreated: async (_: WebviewWindow) => {
    // await ww.setPosition(new LogicalPosition(1600, 840));
  },
  onError: async (_: WebviewWindow, error: any) => {
    Logger.error("an error happened creating the webview", error);
  },
};
