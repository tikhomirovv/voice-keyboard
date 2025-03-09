<script setup lang="ts">
import SettingsShortcutsItem from "./SettingsShortcutsItem.vue";
import { useShortcuts } from "@/composables/useShortcuts";
const { list: shortcuts } = useShortcuts();
</script>
<template>
  <div class="settings-container bg-white rounded-lg shadow-md">
    <div class="mb-6">
      <h2 class="text-xl font-semibold mb-3">Текущие нажатые клавиши</h2>
      <SettingsShortcutsItem
        v-for="sc in shortcuts"
        :key="sc.id"
        :shortcut="sc"
        class="p-4 bg-gray-100 rounded-lg"
      />
    </div>
  </div>
</template>
