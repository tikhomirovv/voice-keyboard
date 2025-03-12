import { invoke } from "@tauri-apps/api/core";
import { PhysicalPosition } from "@tauri-apps/api/dpi";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import Logger from "./logger";

type MonitorSize = {
  size: number[];
  position: number[];
};

async function getMonitorSize(): Promise<MonitorSize> {
  return invoke<MonitorSize>("get_monitor_info");
}

type Position = {
  x?: number;
  y?: number;
};

export async function setPosition(ww: WebviewWindow, position: Position) {
  const [monitorInfo, wwSize, wwPosition] = await Promise.all([
    getMonitorSize(),
    ww.size(),
    ww.outerPosition(),
  ]);

  let finalX = position.x ?? wwPosition.x;
  let finalY = position.y ?? wwPosition.y;

  if (position.x && position.x < 0) {
    finalX = monitorInfo.size[0] - wwSize.width + position.x;
  }

  if (position.y && position.y < 0) {
    finalY = monitorInfo.size[1] - wwSize.height + position.y;
  }

  finalX = Math.max(0, Math.min(finalX, monitorInfo.size[0] - wwSize.width));
  finalY = Math.max(0, Math.min(finalY, monitorInfo.size[1] - wwSize.height));

  Logger.info(`Final position`, { x: finalX, y: finalY });

  await ww.setPosition(new PhysicalPosition(finalX, finalY));
}
