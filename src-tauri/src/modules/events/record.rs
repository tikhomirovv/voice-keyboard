use crate::modules::audio::SampleType;
use crate::utils::get_current_timestamp;
use lazy_static::lazy_static;
use serde::Serialize;
use std::sync::Mutex;
use tauri::ipc::Channel;

lazy_static! {
    static ref GLOBAL_EVENT_CHANNEL_RECORD: Mutex<Option<Channel<RecordEvent>>> = Mutex::new(None);
}

// Функция для получения канала из других модулей
pub fn get_event_channel_record() -> Option<Channel<RecordEvent>> {
    GLOBAL_EVENT_CHANNEL_RECORD.lock().unwrap().clone()
}

// Функция для установки канала
pub fn set_event_channel_record_global(channel: Channel<RecordEvent>) {
    *GLOBAL_EVENT_CHANNEL_RECORD.lock().unwrap() = Some(channel);
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum RecordEvent {
    #[serde(rename_all = "camelCase")]
    Start { timestamp: u64 },
    #[serde(rename_all = "camelCase")]
    Progress { timestamp: u64, peak: SampleType },
    #[serde(rename_all = "camelCase")]
    Stop { timestamp: u64 },
}

impl RecordEvent {
    pub fn start() -> Self {
        RecordEvent::Start {
            timestamp: get_current_timestamp(),
        }
    }
    pub fn progress(peak: SampleType) -> Self {
        RecordEvent::Progress {
            timestamp: get_current_timestamp(),
            peak,
        }
    }
    pub fn stop() -> Self {
        RecordEvent::Stop {
            timestamp: get_current_timestamp(),
        }
    }
    pub fn send(&self) {
        if let Some(channel) = get_event_channel_record() {
            channel.send(self.clone()).unwrap();
        }
    }
}
