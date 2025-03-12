import Logger from "@/lib/system/logger";
import { PhysicalPosition } from "@tauri-apps/api/dpi";
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
    maxHeight: 100,
    maximizable: false,
    closable: false,
    resizable: false,
    minimizable: false,
    transparent: true,
    decorations: true,
    alwaysOnTop: true,
  },
  onCreated: async (ww: WebviewWindow) => {
    // await ww.startDragging();
    // Logger.info("startDragging", ww.label);
    await ww.setPosition(new PhysicalPosition(1600, 800));
  },
  onError: async (_: WebviewWindow, error: any) => {
    Logger.error("an error happened creating the webview", error);
  },
};
