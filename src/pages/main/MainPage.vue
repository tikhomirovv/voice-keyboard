<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("...");

const refresh = async () => {
  return fetchMicrophones();
};
const stopRecord = async () => {
  try {
    const result = await invoke<string>("stop_record");
    greetMsg.value = result;
  } catch (err) {
    greetMsg.value = "Error stopping recording: " + err;
  }
};
const startRecord = async () => {
  if (!selectedMic.value) {
    alert("Выберите микрофон");
    return;
  }

  try {
    await invoke("start_record", {
      deviceId: selectedMic.value,
    });
    //greetMsg.value = result;
  } catch (err) {
    greetMsg.value = "Error starting recording: " + err;
  }
};

// Определяем тип микрофона
interface Microphone {
  id: number;
  name: string;
}

const microphones = ref<Microphone[]>([]);
const selectedMic = ref<number | null>(null);

const fetchMicrophones = async () => {
  try {
    const result = await invoke<string>("get_microphones"); // Приходит JSON-строка
    microphones.value = JSON.parse(result) as Microphone[]; // Парсим JSON в массив объектов

    if (microphones.value.length > 0) {
      selectedMic.value = microphones.value[0].id; // Теперь без ошибки
    }
  } catch (error) {
    console.error("Ошибка загрузки микрофонов:", error);
  }
};

onMounted(fetchMicrophones);
</script>

<template>
  <div class="container">
    <h1>Welcome to Tauri + Vue</h1>

    {{ selectedMic }}

    <div class="row">
      <label for="mic-select">Выберите микрофон:</label>
      <select v-model="selectedMic" class="px-4 py-2 border rounded-lg mb-4">
        <option v-for="mic in microphones" :key="mic.id" :value="mic.id">
          {{ mic.name }}
        </option>
      </select>
      <button @click="refresh" class="btn-secondary">Refresh</button>
      <br />
      <button @click="startRecord" class="btn-primary">Start record</button>
      <button @click="stopRecord" class="btn-danger">Stop</button>
    </div>
    <p>{{ greetMsg }}</p>
  </div>
</template>

<style scoped></style>
<style></style>
