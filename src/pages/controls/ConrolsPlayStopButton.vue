<script lang="ts" setup>
import { useTranscribe } from "@/composables/useTranscribe";
import { PlayIcon, StopIcon } from "@heroicons/vue/24/solid";

defineProps<{
  isRecording: boolean;
}>();

const { stop } = useTranscribe();
</script>

<template>
  <div
    @click="stop"
    :class="{
      'w-0': !isRecording,
      'w-[20px] border ml-2 mr-1': isRecording,
    }"
    class="h-[20px] border-red-500 rounded-full transition-all duration-150 flex justify-center items-center"
  >
    <PlayIcon v-if="!isRecording" class="size-4 text-red-200" />
    <StopIcon v-else class="size-4 text-red-200" />
  </div>
</template>
