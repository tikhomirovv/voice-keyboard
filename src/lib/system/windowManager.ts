import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { WindowOptions, WindowLabel, Window } from "@/types/window";

class WindowManager {
  private async getWindow(label: string): Promise<WebviewWindow | null> {
    return WebviewWindow.getByLabel(label);
  }

  private async createWindow(
    label: WindowLabel,
    options: WindowOptions
  ): Promise<WebviewWindow> {
    return new WebviewWindow(label, options);
  }

  async destroyWindow(w: Window): Promise<void> {
    const window = await this.getWindow(w.label);
    if (window) {
      await window.destroy();
    }
  }
  async initWindowOnce(w: Window): Promise<WebviewWindow> {
    const window = await this.getWindow(w.label);
    if (window) {
      return window;
    }
    const newWindow = await this.createWindow(w.label, w.options);
    newWindow.once("tauri://created", () => w.onCreated(newWindow));
    newWindow.once("tauri://error", (e) => w.onError(newWindow, e));
    return newWindow;
  }
}

export default new WindowManager();
