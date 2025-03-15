import { BackendMessageService } from "@/lib/events/backendMessage";
import { inject } from "vue";

export function useBackendMessageEvents(): BackendMessageService {
  return inject("backendMessageService")!;
}
