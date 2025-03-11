<script setup lang="ts">
import { useMicrophone } from "@/composables/useMicrophone";
const {
  selected: selectedMic,
  microphones,
  refresh,
  set: setMicrophone,
} = useMicrophone();
import AudioVisualizer from "@/components/AudioVisualizer.vue";
// Correcting the event type to Event

function selectMic(event: Event) {
  // Assuming event.target is an HTMLInputElement
  const target = event.target as HTMLInputElement;
  setMicrophone(target.value);
}
</script>

<template>
  <div class="container">
    <h1>Welcome to Tauri + Vue</h1>

    {{ selectedMic }}

    <div class="row">
      <label for="mic-select">Выберите микрофон:</label>
      <select
        @change="selectMic"
        :value="selectedMic"
        class="px-4 py-2 border rounded-lg mb-4"
      >
        <option v-for="mic in microphones" :key="mic.id" :value="mic.id">
          {{ mic.name }}
        </option>
      </select>
      <button @click="refresh" class="btn-secondary">Refresh</button>
      <br />
      <!-- <button @click="startRecord" class="btn-primary">Start record</button> -->
      <!-- <button @click="stopRecord" class="btn-danger">Stop</button> -->
    </div>
    <!-- <p>{{ greetMsg }}</p> -->
    <AudioVisualizer :height="100" :width="100" />
  </div>
</template>

<style scoped></style>
<style></style>
