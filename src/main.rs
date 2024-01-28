use error::MapLog;
use std::env;

mod date_time;
mod error;
mod git_utils;
mod python_status;
mod structs;
mod user_host;
mod util;

fn main() -> error::Result<()> {
    init_app_name();
    let theme_data = structs::ThemeData {
        datetime: date_time::date_time(),
        hostname: user_host::hostname(),
        username: user_host::username(),
        python: python_status::python_info(),
        git: git_utils::process_current_dir(&structs::GetGitInfoOptions::default()).ok_or_log(),
    };

    println!("{:?}", theme_data);
    Ok(())
}

fn init_app_name() {
    let _ = error::APP_NAME.get_or_init(|| {
        env::current_exe()
            .map_or_else(
                |_| Some(env!("CARGO_BIN_NAME").to_string()),
                |p| p.file_stem().map(|s| s.to_string_lossy().to_string()),
            )
            .expect("filename by env")
    });
}
