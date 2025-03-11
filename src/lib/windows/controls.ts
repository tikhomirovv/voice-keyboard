import { WebviewWindow } from "@tauri-apps/api/WebviewWindow";

export const webViewWindow = new WebviewWindow("controls", {
  title: "controls",
  url: "/panels/controls",
  width: 500,
  maxWidth: 500,
  height: 400,
  maxHeight: 500,
  maximizable: false,
  closable: false,
  resizable: false,
  minimizable: false,
  transparent: true,
  decorations: true,
  alwaysOnTop: true,
});

webViewWindow.once("tauri://created", function () {
  console.log("webview successfully created");
  // webview successfully created
});
webViewWindow.once("tauri://error", function (e) {
  console.log("an error happened creating the webview", e);
  // an error happened creating the webview
});
