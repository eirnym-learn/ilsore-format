use crate::error::MapLog;
pub fn hostname() -> String {
    return gethostname::gethostname()
        .to_str()
        .unwrap_or_default()
        .to_string();
}

pub fn username() -> Option<String> {
    return std::env::var("USER").ok_or_log();
}
