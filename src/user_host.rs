pub fn hostname() -> std::ffi::OsString {
    return gethostname::gethostname();
}

pub fn username() -> Option<std::ffi::OsString> {
    return std::env::var_os("USER");
}
