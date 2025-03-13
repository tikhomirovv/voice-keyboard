import type { EventBus } from "@/types/events";
import { MittEventBus } from "./mittEventBus";

let instance: EventBus | null = null;

export function createEventBus<T extends Record<string, any>>(): EventBus<T> {
  if (!instance) {
    instance = new MittEventBus<T>() as EventBus<T>;
  }
  return instance as EventBus<T>;
}
