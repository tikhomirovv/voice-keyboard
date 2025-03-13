<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { useAudioEvents } from "@/composables/useAudioEvents";

const isVisivble = ref(false);
const status = ref<"idle" | "recording">("idle");
const audioEvents = useAudioEvents();

let offStart = () => {};
let offStop = () => {};
onMounted(() => {
  offStart = audioEvents.onStart(() => {
    status.value = "recording";
    isVisivble.value = true;
  });
  offStop = audioEvents.onStop(() => {
    status.value = "idle";
    isVisivble.value = false;
  });
});

onUnmounted(() => {
  offStart();
  offStop();
});
</script>

<template>
  <div
    v-if="isVisivble"
    class="text-white text-xs w-[60px] flex justify-center items-center"
  >
    {{ status }}
  </div>
</template>
