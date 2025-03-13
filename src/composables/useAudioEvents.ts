import { AudioEventService } from "@/services/audio/audioService";
import { inject } from "vue";

export function useAudioEvents(): AudioEventService {
  return inject("audioEventsService")!;
}
