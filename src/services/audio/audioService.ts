import type { EventBus } from "@/types/events";
import type { RecordEvent } from "@/lib/events";
const EVENT_AUDIO_START = "audio:start";
const EVENT_AUDIO_PROGRESS = "audio:progress";
const EVENT_AUDIO_STOP = "audio:stop";

export class AudioEventService {
  constructor(private eventBus: EventBus) {}

  handleRustEvent(event: RecordEvent): void {
    switch (event.event) {
      case "start":
        this.eventBus.emit(EVENT_AUDIO_START, {
          timestamp: event.data.timestamp,
        });
        break;

      case "progress":
        this.eventBus.emit(EVENT_AUDIO_PROGRESS, {
          peak: event.data.peak,
          timestamp: event.data.timestamp,
        });
        break;

      case "stop":
        this.eventBus.emit(EVENT_AUDIO_STOP, {
          timestamp: event.data.timestamp,
        });
        break;
    }
  }

  onStart(handler: (data: { timestamp: number }) => void): () => void {
    this.eventBus.on(EVENT_AUDIO_START, handler);
    return () => this.eventBus.off(EVENT_AUDIO_START, handler);
  }

  onProgress(
    handler: (data: { peak: number; timestamp: number }) => void
  ): () => void {
    this.eventBus.on(EVENT_AUDIO_PROGRESS, handler);
    return () => this.eventBus.off(EVENT_AUDIO_PROGRESS, handler);
  }

  onStop(handler: (data: { timestamp: number }) => void): () => void {
    this.eventBus.on(EVENT_AUDIO_STOP, handler);
    return () => this.eventBus.off(EVENT_AUDIO_STOP, handler);
  }
}
