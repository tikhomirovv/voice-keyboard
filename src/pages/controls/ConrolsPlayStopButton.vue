<script lang="ts" setup>
defineProps<{
  isRecording: boolean;
}>();
</script>

<template>
  <div
    :class="{
      'w-0': !isRecording,
      'w-[20px] border ml-2 mr-1': isRecording,
    }"
    class="h-[20px] border-red-500 rounded-full transition-all duration-150"
  ></div>
</template>
