use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_time() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Couldn't get current time since unix epoch.")
        .as_micros()
}
