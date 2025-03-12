<script setup lang="ts">
import AudioVisualizer from "@/components/controls/AudioVisualizer.vue";
import { onMounted } from "vue";
// import Logger from "@/lib/system/logger";
import { setPosition } from "@/lib/system/monitor";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

onMounted(async () => {
  const ww = getCurrentWebviewWindow();
  await ww.center();
  await setPosition(ww, { y: -50 });
});
</script>

<template>
  <div
    class="bg-transparent absolute z-0 inset-0 flex justify-center items-center"
  >
    <div class="absolute inset-0 z-10" data-tauri-drag-region></div>
    <AudioVisualizer :height="20" :width="60" :compressor="0.2" />
  </div>
</template>

<style scoped></style>
