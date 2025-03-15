import { createEventBus } from "@/lib/events/implementations/factory";
import { invoke } from "@tauri-apps/api/core";
import { AudioEventService, RecordEvent } from "@/lib/events/audio";
import { AudioEventPayload, BackendMessageEventPayload } from "@/types/events";
import { Channel } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  BackendMessageService,
  MessageEvent,
} from "@/lib/events/backendMessage";

// Служба для работы с аудио событиями
export const audioEventService = new AudioEventService(
  createEventBus<AudioEventPayload>()
);
const channel = new Channel<RecordEvent>();
channel.onmessage = (event) => audioEventService.handleRustEvent(event);
invoke("set_event_channel_record", { channel });

// Служба для работы с сообщениями от бэкенда
export const backendMessageService = new BackendMessageService(
  createEventBus<BackendMessageEventPayload>()
);

listen<MessageEvent>("message", (event) => {
  backendMessageService.handleBackendEvent(event.payload);
});
