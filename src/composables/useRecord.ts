import { ref, watch } from "vue";

// Создаем реактивные переменные для хранения данных
const recordingPeak = ref<number[]>([]);
const recordingStatus = ref<"idle" | "recording" | "finished">("idle");
const recordingTimestamp = ref<number>(0);
export const useRecord = () => {
  watch(recordingStatus, (newStatus: String) => {
    if (newStatus == "recording" || newStatus == "finished") {
      recordingPeak.value = [];
    }
  });

  return {
    recordingPeak,
    recordingStatus,
    recordingTimestamp,
  };
};
