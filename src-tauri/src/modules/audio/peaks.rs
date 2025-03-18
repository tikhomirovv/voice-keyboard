lazy_static! {
    static ref LAST_SEND_TIME: Mutex<Instant> = Mutex::new(Instant::now());
}

const THROTTLE_DURATION: Duration = Duration::from_millis(10); // 100 -> 10 раз в секунду

/// Вычисляет пиковое значение из сэмплов и отправляет события о прогрессе
fn process_peaks(samples: &[SampleType]) {
    let current_peak = samples.iter().fold(0 as SampleType, |peak, &sample| {
        if sample > 0 {
            peak.max(sample.min(SampleType::MAX))
        } else {
            peak.min(sample.max(SampleType::MIN))
        }
    });

    // Отправляем событие о прогрессе, если нужно
    let mut last_send = LAST_SEND_TIME.lock().unwrap();
    if last_send.elapsed() >= THROTTLE_DURATION {
        RecordEvent::progress(current_peak).send();
        *last_send = Instant::now();
    }
}
