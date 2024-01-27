use error::MapLog;
use std::env;

mod error;
mod git_utils;
mod structs;
mod util;

fn main() -> error::Result<()> {
    let _ = error::APP_NAME.get_or_init(|| {
        env::current_exe()
            .map_or_else(
                |_| Some(env!("CARGO_BIN_NAME").to_string()),
                |p| p.file_stem().map(|s| s.to_string_lossy().to_string()),
            )
            .expect("filename by env")
    });

    let full_name = Some(String::from("hello world"));
    let _short_name: Option<String> = full_name
        .as_ref()
        .map(|v| v.strip_prefix("refs/heads/").unwrap_or(v))
        .map(|v| String::from(v));

    let a = git_utils::process_current_dir(&structs::GetGitInfoOptions::default()).ok_or_log();
    println!("{:?}", a);
    Ok(())
}
