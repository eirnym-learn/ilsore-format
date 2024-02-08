use crate::error::MapLog;

pub fn hostname() -> Option<String> {
    hostname::get()
        .ok_or_log()
        .and_then(|s| s.into_string().ok())
}

pub fn username() -> Option<String> {
    std::env::var("USER")
        .ok_or_log()
        .or_else(|| std::env::var("USERNAME").ok_or_log())
}
