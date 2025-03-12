<script setup lang="ts">
import { useAudioVisualizer } from "@/composables/useAudioVisualizer";
const props = defineProps<{
  width?: number;
  height?: number;
}>();
const { containerRef, peaks, status, timestamp } = useAudioVisualizer({
  width: props.width,
  height: props.height,
});
</script>

<template>
  <div>
    <div ref="containerRef"></div>
    <div>
      <div>Статус: {{ status }}</div>
      <div>Количество пиков: {{ peaks.length }}</div>
      <div>Пиковое значение: {{ peaks }}</div>
      <div>Время: {{ timestamp }}</div>
    </div>
  </div>
</template>
