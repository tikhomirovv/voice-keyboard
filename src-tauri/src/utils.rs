use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

#[allow(dead_code)]
pub fn is_debug() -> bool {
    let is_debug = std::env::var("IS_DEBUG").unwrap_or_else(|_| "0".to_string());
    is_debug == "1" || is_debug == "true"
}
