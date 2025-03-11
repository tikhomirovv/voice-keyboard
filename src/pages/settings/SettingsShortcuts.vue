<script setup lang="ts">
import SettingsShortcutsItem from "./SettingsShortcutsItem.vue";
import { useShortcuts } from "@/composables/useShortcuts";
const { list: shortcuts, refresh } = useShortcuts();

const handleShortcutUpdated = async () => {
  await refresh();
};
const handleShortcutDeleted = async () => {
  await refresh();
};
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
        @shortcutDeleted="handleShortcutDeleted"
        @shortcutUpdated="handleShortcutUpdated"
      />
    </div>
  </div>
</template>
