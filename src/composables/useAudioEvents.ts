import { AudioEventService } from "@/lib/events/audio";
import { inject } from "vue";

export function useAudioEvents(): AudioEventService {
  return inject("audioEventsService")!;
}
