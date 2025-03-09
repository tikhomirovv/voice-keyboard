<script setup lang="ts">
import { ref, watch } from "vue";

import { useShortcuts } from "@/composables/useShortcuts";
import type { Shortcut } from "@/types/shortcuts";

const props = defineProps<{
  shortcut: Shortcut;
}>();

const { lastPressedKeys, isPressing, updateShortcut } = useShortcuts();
const isEditing = ref(false);

function update(isPressing: Boolean) {
  if (!isPressing && isEditing.value) {
    const newKey = Array.from(lastPressedKeys.value).join("+");
    console.log("Update shortcut", props.shortcut.id, newKey);
    updateShortcut(props.shortcut.id, newKey);
    isEditing.value = false;
  }
}

watch(isPressing, update);
</script>

<template>
  <div>
    <div>{{ shortcut.name }}</div>
    <div>{{ shortcut.description }}</div>
    <button
      @click="isEditing = !isEditing"
      class="mt-2 mb-2 px-4 py-2 bg-blue-500 text-white rounded-md"
    >
      {{ isEditing ? "Stop Editing" : "Edit Shortcut" }}
    </button>
    <div
      v-if="isEditing && lastPressedKeys.size > 0"
      class="flex flex-wrap gap-2"
    >
      <span
        v-for="key in lastPressedKeys"
        :key="key"
        class="px-3 py-1 bg-blue-500 text-white rounded-md"
      >
        {{ key }}
      </span>
    </div>
    <p v-else-if="isEditing" class="text-gray-500">
      Нажмите любую клавишу или комбинацию клавиш...
    </p>
    <div v-else class="text-gray-500">
      {{ shortcut.key }}
    </div>
  </div>
</template>
