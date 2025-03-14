import { renderMicStream } from "@/lib/audiowave";
import { ref, onMounted, onUnmounted } from "vue";
import type { MicStream } from "@/lib/audiowave";
import { useAudioEvents } from "@/composables/useAudioEvents";
import { preview } from "vite";

const MAX_INT_16 = 32767;
const MAX_INT_8 = 128;
const BITS_PER_SAMPLE: 8 | 16 = 8; // 8-bit or 16-bit audio
const MAX_INT = BITS_PER_SAMPLE ? MAX_INT_8 : MAX_INT_16;

export function useAudioVisualizer(options: {
  width?: number;
  height?: number;
  color?: string;
  compressorRatio?: number; // меньше 1 -> больше компрессии, больше 1 -> expander (обратный эффект)
}) {
  let micStream: MicStream | null = null;
  const containerRef = ref<HTMLDivElement | null>(null);
  const compressorRatio = options.compressorRatio || 1;

  const peaks = ref<number[]>([]);
  const status = ref<"idle" | "recording">("idle");
  const timestamp = ref<number>(0);
  const audioEvents = useAudioEvents();

  onMounted(() => {
    micStream = renderMicStream({
      containerRef: containerRef.value!,
      width: options.width,
      height: options.height,
      color: options.color,
      scrollingWaveformWindow: 15,
    });
  });

  onUnmounted(() => {
    offStart();
    offProgress();
    offStop();
    micStream?.onDestroy();
  });

  const offStart = audioEvents.onStart(({ timestamp: ts }) => {
    micStream?.onUpdate([]);
    peaks.value = [];
    status.value = "recording";
    timestamp.value = ts;
  });

  const offProgress = audioEvents.onProgress(({ timestamp: ts, peak }) => {
    status.value = "recording";
    peak = peak / MAX_INT;
    peak = noSilent(peak, 0.03);
    peak = compressor(peak, compressorRatio);
    peaks.value.push(peak);
    timestamp.value = ts;
    micStream?.onUpdate(peaks.value);
  });

  const offStop = audioEvents.onStop(({ timestamp: ts }) => {
    status.value = "idle";
    timestamp.value = ts;
    micStream?.onUpdate([]);
    peaks.value = [];
  });

  return {
    containerRef,
    peaks,
    status,
    timestamp,
  };
}

// Функция compressor изменяет значение, увеличивая его, если оно близко к нулю.
function compressor(value: number, ratio: number = 0.5): number {
  return Math.sign(value) * Math.abs(value) ** ratio;
}
// Если значение близко к нулю, функция возвращает случайное число
function noSilent(value: number, noize: number): number {
  if (value === 0) {
    return Math.random() * noize * 2 - noize;
  }
  return value;
}
