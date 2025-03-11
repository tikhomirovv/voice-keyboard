<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from "vue";
import WaveSurfer from "wavesurfer.js";
import { useRecord } from "@/composables/useRecord";
import { renderMicStream } from "@/lib/audiowave";
import type { MicStream } from "@/lib/audiowave";
const { recordingStatus, recordingPeak } = useRecord();
import { watchDeep } from "@vueuse/core";

const props = defineProps<{
  width?: number;
  height?: number;
}>();

const containerRef = ref<HTMLDivElement | null>(null);
let wavesurfer: WaveSurfer | null = null;
let micStream: MicStream | null = null;

// Следим за статусом записи
watch(recordingStatus, (newStatus) => {
  if (newStatus === "recording" && wavesurfer) {
    // Создаем новый стрим только при начале записи
    micStream?.onDestroy(); // На всякий случай очищаем предыдущий
    micStream = renderMicStream({
      wavesurfer,
      scrollingWaveformWindow: 20,
    });
  }
});

watchDeep(recordingPeak, (newPeak) => {
  micStream?.onUpdate(newPeak);
});

onMounted(() => {
  // Создаем экземпляр WaveSurfer
  wavesurfer = WaveSurfer.create({
    container: containerRef.value!,
    waveColor: "#4F4A85",
    progressColor: "#383351",
    width: props.width || 300,
    height: props.height || 100,
    cursorWidth: 0,
    barWidth: 10,
    barRadius: 4,
    interact: false,
    minPxPerSec: 1,
    // Отключаем загрузку аудио файла
    url: "",
    // peaks: [recordingPeak.value],
    // duration: recordingPeak.value.length,
  });
  // wavesurfer.params.peaks = recordingPeak.value;
});

onUnmounted(() => {
  micStream?.onDestroy();
  wavesurfer?.destroy();
});
</script>

<template>
  <div>
    <div ref="containerRef"></div>
    <div>
      <div>Статус: {{ recordingStatus }}</div>
      <!-- <div>Количество пиков: {{ recordingPeak.length }}</div> -->
      <!-- <div>Пиковое значение: {{ recordingPeak }}</div> -->
    </div>
  </div>
</template>
