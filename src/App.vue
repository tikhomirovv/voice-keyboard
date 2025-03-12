<script setup lang="ts">
import { RouterView, RouterLink, useRoute } from "vue-router";
const route = useRoute();
</script>

<template>
  <div class="min-h-screen bg-gray-100">
    <nav class="bg-white shadow-md p-4">
      <div class="container mx-auto flex justify-between items-center">
        <RouterLink to="/" class="text-xl font-bold text-blue-600"
          >Voice Keyboard</RouterLink
        >
        <div class="flex items-center gap-4">
          <div>{{ route.fullPath }}</div>
          <RouterLink to="/" class="text-gray-600 hover:text-blue-600"
            >Главная</RouterLink
          >
          <RouterLink to="/settings" class="text-gray-600 hover:text-blue-600"
            >Настройки</RouterLink
          >
        </div>
      </div>
    </nav>

    <main class="container mx-auto px-4 py-8">
      <RouterView />
    </main>
  </div>
</template>
