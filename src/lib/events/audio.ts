import type { AudioEventPayload, EventBus } from "@/types/events";
import Logger from "@/lib/system/logger";

const EVENT_AUDIO_START = "start";
const EVENT_AUDIO_PROGRESS = "progress";
const EVENT_AUDIO_STOP = "stop";

export class AudioEventService {
  constructor(private eventBus: EventBus<AudioEventPayload>) {}

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

export type RecordEvent =
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
    };
