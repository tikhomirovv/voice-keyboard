<script setup lang="ts">
import AudioVisualizer from "@/components/controls/AudioVisualizer.vue";
import { onMounted } from "vue";
import { setPosition } from "@/lib/system/monitor";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

onMounted(async () => {
  const ww = getCurrentWebviewWindow();
  await ww.center();
  await setPosition(ww, { y: -57 });
});
</script>

<template>
  <div
    class="bg-transparent absolute z-0 inset-0 flex justify-center items-center"
  >
    <div class="absolute inset-0 z-20" data-tauri-drag-region></div>
    <div
      class="relative bg-black/70 w-[70px] rounded-full h-[28px] overflow-hidden flex justify-center items-center border border-white/20"
    >
      <AudioVisualizer
        :height="14"
        :width="40"
        :color="'#ffffffcc'"
        :compressor="0.2"
        class="mix-blend-screen"
      />
    </div>
  </div>
</template>

<style scoped></style>
