<script setup lang="ts">
import { useAudioVisualizer } from "@/composables/useAudioVisualizer";
const props = defineProps<{
  width?: number;
  height?: number;
  compressor?: number;
}>();
const { containerRef } = useAudioVisualizer({
  width: props.width,
  height: props.height,
  compressor: props.compressor,
});
</script>

<template>
  <div>
    <div ref="containerRef"></div>
  </div>
</template>
