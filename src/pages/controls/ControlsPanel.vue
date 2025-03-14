<script setup lang="ts">
// import AudioRecordStatus from "@/components/controls/AudioRecordStatus.vue";
import AudioVisualizer from "@/components/controls/AudioVisualizer.vue";
import { onMounted, onUnmounted, ref, computed } from "vue";
import { useAudioEvents } from "@/composables/useAudioEvents";
import ControlsPlayStopButton from "@/pages/controls/ConrolsPlayStopButton.vue";
import ControlsCancelButton from "@/pages/controls/ConrolsCancelButton.vue";
const status = ref<"idle" | "recording">("idle");
const audioEvents = useAudioEvents();
const isRecording = computed(() => status.value === "recording");

onMounted(() => {
  const offStart = audioEvents.onStart(() => {
    status.value = "recording";
  });
  const offStop = audioEvents.onStop(() => {
    status.value = "idle";
  });

  onUnmounted(() => {
    offStart();
    offStop();
  });
});
</script>

<template>
  <div class="flex flex-col overflow-visible justify-center items-center">
    <div class="absolute inset-0 z-20" data-tauri-drag-region></div>
    <div
      class="relative overflow-visible bg-black/70 min-w-[80px] h-[32px] rounded-full justify-center items-center border border-white/20 flex text-white"
    >
      <ControlsPlayStopButton :is-recording="isRecording" />
      <AudioVisualizer
        :height="14"
        :width="40"
        :color="'#ffffffcc'"
        :compressor-ratio="0.4"
        class="w-[40px]"
      />
      <ControlsCancelButton :is-recording="isRecording" />
    </div>
  </div>
</template>

<style scoped></style>
