<script setup lang="ts">
import { useAudioVisualizer } from "@/composables/useAudioVisualizer";
const props = defineProps<{
  width?: number;
  height?: number;
  color?: string;
  compressor?: number;
}>();
const { containerRef } = useAudioVisualizer({
  width: props.width,
  height: props.height,
  color: props.color,
  compressor: props.compressor,
});
</script>

<template>
  <div>
    <div ref="containerRef"></div>
  </div>
</template>
