import mitt from "mitt";
import type { EventBus, EventHandler } from "@/types/events";

export class MittEventBus<T extends Record<string, any>>
  implements EventBus<T>
{
  private emitter = mitt<T>();

  emit<K extends keyof T>(event: K, payload: T[K]): void {
    this.emitter.emit(event, payload);
  }
  on<K extends keyof T>(event: K, handler: EventHandler<T[K]>): void {
    this.emitter.on(event, handler);
  }
  off<K extends keyof T>(event: K, handler?: EventHandler<T[K]>): void {
    if (handler) {
      this.emitter.off(event, handler);
    } else {
      this.emitter.off(event);
    }
  }
  clear(): void {
    this.emitter.all.clear();
  }
}
