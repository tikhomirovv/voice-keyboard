<script lang="ts" setup>
import { XMarkIcon } from "@heroicons/vue/24/solid";
defineProps<{
  isRecording: boolean;
}>();
</script>

<template>
  <div
    :class="{
      'w-0': !isRecording,
      'w-[20px] border mr-2 ml-1': isRecording,
    }"
    class="h-[20px] border-red-500 rounded-full transition-all duration-150 flex justify-center items-center"
  >
    <XMarkIcon class="size-4 text-red-200" />
  </div>
</template>
