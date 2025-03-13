import "@/assets/main.css";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { init as initApp } from "./entrypoint/app/app";
import { init as initControls } from "./entrypoint/controls/controls";

const currentWindow = getCurrentWindow();
switch (currentWindow.label) {
  case "main":
    initApp();
    break;
  case "controls":
    initControls();
    break;
}
