<script setup lang="ts">
import { onMounted, onUnmounted } from "vue";
import { setPosition } from "@/lib/system/monitor";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
// import { useResizeObserver } from "@vueuse/core";
import { useTemplateRef } from "vue";
import { LogicalSize } from "@tauri-apps/api/window";
// import Logger from "@/lib/system/logger";

const WINDOW_SIZE = new LogicalSize(110, 32);
const WINDOW_POSITION = { y: -57 };

const containerRef = useTemplateRef<HTMLDivElement>("containerRef");
const ww = getCurrentWebviewWindow();

(async () => {
  await ww.setSize(WINDOW_SIZE);
  await ww.center();
  await setPosition(ww, WINDOW_POSITION);

  // useResizeObserver(containerRef, async (entries) => {
  //   const entry = entries[0];
  //   const { width, height } = entry.contentRect;
  //   Logger.debug(`width: ${width}, height: ${height}`);
  //   // await new Promise((resolve) => setTimeout(resolve, 1000));
  //   await ww.setSize(new LogicalSize(width, height));
  //   await setPosition(ww, WINDOW_POSITION);
  // });
})();
onMounted(() => {});
onUnmounted(() => {});
</script>

<template>
  <main ref="containerRef">
    <RouterView />
  </main>
</template>

<style>
body {
  @apply overflow-hidden;
}
</style>
