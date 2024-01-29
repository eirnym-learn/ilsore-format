use error::MapLog;
use std::env;
use std::thread;

mod date_time;
mod error;
mod git_utils;
mod ilsore_format;
mod python_status;
mod structs;
mod user_host;
mod util;

fn main() -> error::Result<()> {
    init_app_name();
    let theme_data = theme_data();
    let symbols = structs::ThemeSymbols::utf_power();
    println!(
        "{}",
        ilsore_format::format_ilsore_no_color(&theme_data, &symbols)
    );
    Ok(())
}

fn theme_data() -> structs::ThemeData {
    let mut hostname: Option<String> = Some(Default::default());
    let mut git_info: Option<Option<structs::GitOutputOptions>> = Some(None);
    thread::scope(|s| {
        s.spawn(|| {
            let _ = hostname.insert(user_host::hostname());
        });
        s.spawn(|| {
            let _ = git_info.insert(
                git_utils::process_current_dir(&structs::GetGitInfoOptions::default()).ok_or_log(),
            );
        });
    });

    structs::ThemeData {
        datetime: date_time::date_time(),
        hostname,
        username: user_host::username(),
        python: python_status::python_info(),
        git: git_info.flatten(),
    }
}
fn init_app_name() {
    let _ = error::APP_NAME.get_or_init(|| {
        if error::VERBOSE_ERRORS {
            env::current_exe()
                .map_or_else(
                    |_| Some(env!("CARGO_BIN_NAME").to_string()),
                    |p| p.file_stem().map(|s| s.to_string_lossy().to_string()),
                )
                .expect("filename by env")
        } else {
            "".to_string()
        }
    });
}
