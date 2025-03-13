<script setup lang="ts">
import { useAudioVisualizer } from "@/composables/useAudioVisualizer";
const props = defineProps<{
  width?: number;
  height?: number;
  color?: string;
  compressorRatio?: number;
}>();
const { containerRef } = useAudioVisualizer({
  width: props.width,
  height: props.height,
  color: props.color,
  compressorRatio: props.compressorRatio,
});
</script>

<template>
  <div class="flex justify-center items-center">
    <div ref="containerRef"></div>
  </div>
</template>
