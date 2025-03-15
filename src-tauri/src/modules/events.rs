use serde::Serialize;

use crate::modules::audio::SampleType;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum RecordEvent {
    #[serde(rename_all = "camelCase")]
    Start { timestamp: u128 },
    #[serde(rename_all = "camelCase")]
    Progress { timestamp: u128, peak: SampleType },
    #[serde(rename_all = "camelCase")]
    Stop { timestamp: u128 },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum Message {
    #[serde(rename_all = "camelCase")]
    Error {
        code: u8,
        code_str: String,
        message: String,
        timestamp: u128,
    },
}
