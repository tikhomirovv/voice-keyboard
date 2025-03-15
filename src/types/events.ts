export type EventHandler<T = any> = (payload: T) => void;
export interface EventBus<T = any> {
  emit<K extends keyof T>(event: K, payload: T[K]): void;
  on<K extends keyof T>(event: K, handler: EventHandler<T[K]>): void;
  off<K extends keyof T>(event: K, handler?: EventHandler<T[K]>): void;
  clear(): void;
}

export type AudioEventPayload = {
  start: { timestamp: number };
  progress: { timestamp: number; peak: number };
  stop: { timestamp: number };
};

export type BackendMessageEventPayload = {
  error: {
    code: number;
    code_str: string;
    message: string;
    timestamp: number;
  };
};
