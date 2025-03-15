import { BackendMessageEventPayload, EventBus } from "@/types/events";
import Logger from "@/lib/system/logger";

const EVENT_MESSAGE_ERROR = "error";

export class BackendMessageService {
  constructor(private eventBus: EventBus<BackendMessageEventPayload>) {}

  handleBackendEvent(event: MessageEvent): void {
    Logger.debug("Backend event: ", event);
    switch (event.type) {
      case "error":
        this.eventBus.emit(EVENT_MESSAGE_ERROR, event.data);
        break;
    }
  }

  onError(handler: (message: ErrorMessageEventPayload) => void): () => void {
    this.eventBus.on(EVENT_MESSAGE_ERROR, handler);
    return () => this.eventBus.off(EVENT_MESSAGE_ERROR, handler);
  }
}

export type ErrorMessageEventPayload = {
  code: number;
  code_str: string;
  message: string;
  timestamp: number;
};

export type MessageType = "error";
export type MessageEvent = {
  type: "error";
  data: ErrorMessageEventPayload;
};
