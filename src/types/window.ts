import {
  WebviewOptions as TauriWebviewOptions,
  WebviewLabel,
} from "@tauri-apps/api/webview";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { WindowOptions as TauriWindowOptions } from "@tauri-apps/api/window";

export type WindowOptions = Omit<
  TauriWebviewOptions,
  "x" | "y" | "width" | "height"
> &
  TauriWindowOptions;

export type WindowLabel = WebviewLabel;

export interface Window {
  label: string;
  options: WindowOptions;
  onCreated: (ww: WebviewWindow) => void;
  onError: (ww: WebviewWindow, error: any) => void;
}
