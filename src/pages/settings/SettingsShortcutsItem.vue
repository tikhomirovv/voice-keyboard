<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { useShortcuts } from "@/composables/useShortcuts";
import type { Shortcut } from "@/types/shortcuts";
import { normalizeKey } from "@/lib/shortcuts";

const props = defineProps<{
  shortcut: Shortcut;
}>();
const emit = defineEmits<{
  (e: "shortcutUpdated"): void;
  (e: "shortcutDeleted"): void;
}>();
const { lastPressedKeys, isPressing, updateShortcut, deleteShortcut } =
  useShortcuts();
const isEditing = ref(false);

// Computed property that returns either lastPressedKeys or split shortcut.key
const displayKeys = computed(() => {
  if (!isEditing.value) {
    return new Set(props.shortcut.key.split("+"));
  }
  return lastPressedKeys.value;
});

async function update(isPressing: Boolean) {
  if (!isPressing && isEditing.value) {
    const newKey = normalizeKey(Array.from(lastPressedKeys.value).join("+"));
    console.log("Update shortcut", props.shortcut.id, newKey);
    try {
      await updateShortcut(props.shortcut.id, newKey);
      handleEdit();
      emit("shortcutUpdated");
    } catch (error) {
      lastPressedKeys.value = new Set();
      console.error("Ошибка при обновлении горячей клавиши:", error);
    }
  }
}

async function handleDelete() {
  // if (confirm("Вы уверены, что хотите удалить эту горячую клавишу?")) {
  await deleteShortcut(props.shortcut.id);
  emit("shortcutDeleted");
  // }
}

function handleEdit() {
  lastPressedKeys.value = new Set();
  isEditing.value = !isEditing.value;
}

watch(isPressing, update);
</script>

<template>
  <div>
    <div>{{ shortcut.name }}</div>
    <div>{{ shortcut.description }}</div>
    <div class="flex gap-2">
      <button
        @click="handleEdit"
        class="mt-2 mb-2 px-4 py-2 bg-blue-500 text-white rounded-md"
      >
        {{ isEditing ? "Stop Editing" : "Edit Shortcut" }}
      </button>
      <button
        @click="handleDelete"
        class="mt-2 mb-2 px-4 py-2 bg-red-500 text-white rounded-md"
      >
        Удалить
      </button>
    </div>
    <div v-if="displayKeys.size > 0" class="flex flex-wrap gap-2">
      <span
        v-for="key in displayKeys"
        :key="key"
        class="px-3 py-1 bg-blue-500 text-white rounded-md"
      >
        {{ key.toUpperCase() }}
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
