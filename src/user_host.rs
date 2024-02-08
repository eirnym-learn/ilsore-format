use crate::error::MapLog;
pub fn hostname() -> String {
    gethostname::gethostname().to_string_lossy().to_string()
}

pub fn username() -> Option<String> {
    std::env::var("USER").ok_or_log()
}
