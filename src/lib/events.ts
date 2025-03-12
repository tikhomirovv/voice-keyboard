export type RecordEvent =
  | {
      event: "start";
      data: {
        timestamp: number;
      };
    }
  | {
      event: "progress";
      data: {
        timestamp: number;
        peak: number;
      };
    }
  | {
      event: "stop";
      data: {
        timestamp: number;
      };
    }
  | {
      event: "clipping";
      data: {
        timestamp: number;
        originalValue: number;
        clippedValue: number;
      };
    };
