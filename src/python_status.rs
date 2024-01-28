use crate::error::MapLog;
use crate::util::LastPart;
use std::env;

pub fn python_info() -> Option<String> {
    env::var("VIRTUAL_ENV")
        .ok_or_log()
        .map(|v| v.as_str().last_two_parts().to_string())
}
