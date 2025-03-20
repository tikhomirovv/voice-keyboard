use crate::modules::audio::SampleType;
use crate::modules::events::record::RecordEvent;
use std::time::{Duration, Instant};
use tokio::sync::broadcast;

const THROTTLE_DURATION: Duration = Duration::from_millis(10); // 100 -> 10 раз в секунду

pub async fn send_peaks(mut peaks_rx: broadcast::Receiver<Vec<SampleType>>) {
    let mut last_send_time = Instant::now();

    while let Ok(samples) = peaks_rx.recv().await {
        let current_peak = samples.iter().fold(0 as SampleType, |peak, &sample| {
            if sample > 0 {
                peak.max(sample.min(SampleType::MAX))
            } else {
                peak.min(sample.max(SampleType::MIN))
            }
        });
        if last_send_time.elapsed() >= THROTTLE_DURATION {
            RecordEvent::progress(current_peak).send();
            last_send_time = Instant::now();
        }
    }
}
