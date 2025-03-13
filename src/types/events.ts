export type AudioEventPayload = {
  "audio:start": { timestamp: number };
  "audio:progress": { timestamp: number; peak: number };
  "audio:stop": { timestamp: number };
};

export type EventHandler<T = any> = (payload: T) => void;
export interface EventBus<T = any> {
  emit<K extends keyof T>(event: K, payload: T[K]): void;
  on<K extends keyof T>(event: K, handler: EventHandler<T[K]>): void;
  off<K extends keyof T>(event: K, handler?: EventHandler<T[K]>): void;

  clear(): void;
}
