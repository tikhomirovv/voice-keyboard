<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("...");

const refresh = async () => {
  return fetchMicrophones();
};
const stopRecord = async () => {
  try {
    await invoke("stop_record");
    greetMsg.value = "Recording stopped";
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
    const result = await invoke<string>("start_record", {
      deviceId: selectedMic.value,
    });
    greetMsg.value = result;
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
  <main class="container">
    <h1>Welcome to Tauri + Vue</h1>

    {{ selectedMic }}

    <div class="row">
      <label for="mic-select">Выберите микрофон:</label>
      <select v-model="selectedMic">
        <option v-for="mic in microphones" :key="mic.id" :value="mic.id">
          {{ mic.name }}
        </option>
      </select>
      <!-- <button @click="startRecord"></button> -->
      <!-- <input id="greet-input" v-model="name" placeholder="Enter a name..." /> -->
      <button @click="refresh">Refresh</button>
      <br />
      <button @click="startRecord">Start record</button>
      <button @click="stopRecord">Stop</button>
    </div>
    <p>{{ greetMsg }}</p>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
