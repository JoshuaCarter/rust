use chrono::*;
use std::{thread, time::Duration};

pub fn now_ms() -> u64 {
    return Local::now().timestamp_millis() as u64;
}

pub fn sleep(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}
