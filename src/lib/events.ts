import { Channel } from "@tauri-apps/api/core";
import { useRecord } from "@/composables/useRecord";

type RecordEvent =
  | {
      event: "start";
      data: {
        timestamp: number;
      };
    }
  | {
      event: "progress";
      data: {
        timestamp: number;
        peak: number;
      };
    }
  | {
      event: "stop";
      data: {
        timestamp: number;
      };
    }
  | {
      event: "clipping";
      data: {
        timestamp: number;
        originalValue: number;
        clippedValue: number;
      };
    };

let currentChannel: Channel<RecordEvent> | null = null;

const { recordingStatus, recordingTimestamp, recordingPeak } = useRecord();
export const createRecordEventChannel = () => {
  // Очищаем предыдущий канал, если он существует
  if (currentChannel) {
    currentChannel.onmessage = () => {}; // пустая функция вместо null
    currentChannel = null;
  }

  // Создаем новый канал
  const channel = new Channel<RecordEvent>();
  channel.onmessage = (message) => {
    switch (message.event) {
      case "start":
        recordingStatus.value = "recording";
        recordingTimestamp.value = message.data.timestamp;
        break;
      case "progress":
        // Конвертируем каждое значение сразу при получении
        const MAX_INT_16 = 32767;
        const float32Value = message.data.peak / MAX_INT_16;
        recordingPeak.value.push(float32Value);
        // recordingPeak.value.push(message.data.peak);
        recordingTimestamp.value = message.data.timestamp;
        break;
      case "stop":
        recordingStatus.value = "finished";
        recordingTimestamp.value = message.data.timestamp;
        break;
    }
  };

  currentChannel = channel;
  return channel;
};
