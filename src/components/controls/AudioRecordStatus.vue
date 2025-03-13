<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { useAudioEvents } from "@/composables/useAudioEvents";

const status = ref<"idle" | "recording">("idle");

const audioEvents = useAudioEvents();

let offStart = () => {};
let offStop = () => {};
onMounted(() => {
  offStart = audioEvents.onStart(() => {
    status.value = "recording";
  });
  offStop = audioEvents.onStop(() => {
    status.value = "idle";
  });
});

onUnmounted(() => {
  offStart();
  offStop();
});
</script>

<template>
  <div class="text-white text-xs">{{ status }}</div>
</template>
