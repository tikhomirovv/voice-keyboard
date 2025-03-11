export type MicStream = {
  onDestroy: () => void;
  onEnd: () => void;
  onUpdate: (peaks: number[]) => void;
};

// Constants for visualization
const DEFAULT_WINDOW_SIZE = 1000; // 1 second at 1000 samples per second

interface RenderMicStreamOptions {
  wavesurfer: any; // Экземпляр WaveSurfer
  scrollingWaveformWindow?: number; // Размер окна для прокрутки
}

export function renderMicStream(options: RenderMicStreamOptions): MicStream {
  let isWaveformPaused = false;
  let dataWindow: Float32Array | null = null;
  let originalOptions: any = null;

  if (options.wavesurfer) {
    originalOptions = {
      ...options.wavesurfer.options,
    };
    options.wavesurfer.options.interact = false;
  }

  const drawWaveform = (peaks: number[]) => {
    if (isWaveformPaused) {
      return;
    }
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
    if (options.wavesurfer) {
      options.wavesurfer
        .load("", [dataWindow], options.scrollingWaveformWindow)
        .catch((err: Error) => {
          console.error("[drawWaveform] Error rendering waveform:", err);
        });
    }
  };

  return {
    onDestroy: () => {
      if (options.wavesurfer && originalOptions) {
        options.wavesurfer.setOptions(originalOptions);
      }
    },
    onEnd: () => {
      isWaveformPaused = true;
    },
    onUpdate: (peaks: number[]) => {
      drawWaveform(peaks);
    },
  };
}
