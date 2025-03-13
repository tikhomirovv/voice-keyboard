import { createEventBus } from "@/lib/events/implementations/factory";
import { invoke } from "@tauri-apps/api/core";
import { AudioEventService } from "@/services/audio/audioService";
import { AudioEventPayload } from "@/types/events";
import { Channel } from "@tauri-apps/api/core";
import { RecordEvent } from "@/lib/events";

const eventBus = createEventBus<AudioEventPayload>();
export const audioEventService = new AudioEventService(eventBus);

const channel = new Channel<RecordEvent>();
channel.onmessage = (event) => audioEventService.handleRustEvent(event);
invoke("set_event_channel_record", { channel });
