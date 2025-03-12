import Logger from "@/lib/system/logger";
import WaveSurfer from "wavesurfer.js";

export type MicStream = {
  onDestroy: () => void;
  onUpdate: (peaks: number[]) => void;
};

// Constants for visualization
const DEFAULT_WINDOW_SIZE = 1000; // 1 second at 1000 samples per second

interface RenderMicStreamOptions {
  containerRef: HTMLDivElement | string;
  width?: number;
  height?: number;
  scrollingWaveformWindow?: number; // Размер окна для прокрутки
}

export function renderMicStream(options: RenderMicStreamOptions): MicStream {
  let dataWindow: Float32Array | null = null;

  const wavesurfer = WaveSurfer.create({
    container: options.containerRef,
    waveColor: "#4F4A85",
    progressColor: "#383351",
    width: options.width || 300,
    height: options.height || 100,
    cursorWidth: 0,
    barWidth: 2,
    barRadius: 4,
    interact: false,
    minPxPerSec: 1,
    url: "",
  });

  const drawWaveform = (peaks: number[]) => {
    const bufferLength = peaks.length;
    const windowSize = Math.floor(
      options.scrollingWaveformWindow || DEFAULT_WINDOW_SIZE
    );
    const tempArray = new Float32Array(windowSize);

    const startIdx = Math.min(0, windowSize - bufferLength);
    const offset = Math.max(windowSize - bufferLength, 0);
    tempArray.set(peaks.slice(-startIdx), offset);
    dataWindow = tempArray;

    // Render the waveform
    if (wavesurfer) {
      wavesurfer
        .load("", [dataWindow], options.scrollingWaveformWindow)
        .catch((err: Error) => {
          Logger.error("[drawWaveform] Error rendering waveform:", err);
        });
    }
  };

  return {
    onDestroy: () => {
      wavesurfer.destroy();
    },
    onUpdate: (peaks: number[]) => {
      drawWaveform(peaks);
    },
  };
}
