import { renderMicStream } from "@/lib/audiowave";
import type { RecordEvent } from "@/lib/events";
import { Channel } from "@tauri-apps/api/core";
import { ref, onMounted, onUnmounted } from "vue";
import type { MicStream } from "@/lib/audiowave";
import { invoke } from "@tauri-apps/api/core";

export function useAudioVisualizer(options: {
  width?: number;
  height?: number;
  color?: string;
  compressor?: number; // меньше 1 -> больше компрессии, больше 1 -> expander (обратный эффект)
}) {
  let currentChannel: Channel<RecordEvent> | null = null;
  let micStream: MicStream | null = null;
  const containerRef = ref<HTMLDivElement | null>(null);
  const peaks = ref<number[]>([]);
  const status = ref<"idle" | "recording">("idle");
  const timestamp = ref<number>(0);

  onMounted(() => {
    const channel = new Channel<RecordEvent>();
    channel.onmessage = onEvent;
    invoke("set_event_channel_record", { channel });

    micStream = renderMicStream({
      containerRef: containerRef.value!,
      width: options.width,
      height: options.height,
      color: options.color,
      scrollingWaveformWindow: 15,
    });
  });

  onUnmounted(() => {
    if (currentChannel) {
      currentChannel.onmessage = () => {};
      currentChannel = null;
    }
    micStream?.onDestroy();
  });

  const compressor = options.compressor || 1;
  function onEvent(message: RecordEvent) {
    switch (message.event) {
      case "start":
        micStream?.onUpdate([]);
        peaks.value = [];
        status.value = "recording";
        timestamp.value = message.data.timestamp;
        break;
      case "progress":
        status.value = "recording";
        // Конвертируем каждое значение сразу при получении
        const MAX_INT_16 = 32767;
        const float32Value = (message.data.peak / MAX_INT_16) * 10;
        const amplifiedValue = amplifyNearZero(float32Value, compressor);
        peaks.value.push(amplifiedValue);
        timestamp.value = message.data.timestamp;
        micStream?.onUpdate(peaks.value);
        break;
      case "stop":
        status.value = "idle";
        timestamp.value = message.data.timestamp;
        micStream?.onUpdate([]);
        peaks.value = [];
        break;
    }
  }

  return {
    containerRef,
    peaks,
    status,
    timestamp,
  };
}

// Функция amplifyNearZero изменяет значение, увеличивая его, если оно близко к нулю.
function amplifyNearZero(value: number, power: number = 0.5): number {
  const sign = Math.sign(value);
  return sign * Math.pow(Math.abs(value), power);
}
