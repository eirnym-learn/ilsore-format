use clap::Parser;
use error::MapLog;
use std::thread;

mod args;
mod date_time;
mod error;
mod git_utils;
mod ilsore_format;
mod ilsore_format_color;
mod python_status;
mod structs;
mod user_host;
mod util;

fn main() -> error::Result<()> {
    error::setup_errors();
    args::init_theme_symbols();
    let args = args::Cli::parse();

    let theme_data = theme_data(&args);
    let symbols = args.symbols();

    let result = if args.color {
        ilsore_format_color::format_ilsore_color(&theme_data, symbols)
    } else {
        ilsore_format::format_ilsore_no_color(&theme_data, symbols)
    };
    print!("{}", result);

    Ok(())
}

fn theme_data(args: &args::Cli) -> structs::ThemeData {
    let mut mut_hostname: Option<String> = Some(Default::default());
    let mut git_info: Option<Option<structs::GitOutputOptions>> = Some(None);

    if args.static_hostname.is_none() || !args.disable_git {
        thread::scope(|s| {
            s.spawn(|| {
                if args.static_hostname.is_none() {
                    let _ = mut_hostname.insert(user_host::hostname());
                }
            });

            s.spawn(|| {
                if !args.disable_git {
                    let _ = git_info.insert(
                        git_utils::process_current_dir(&structs::GetGitInfoOptions::default())
                            .ok_or_log(),
                    );
                }
            });
        });
    }
    let hostname = if args.static_hostname.is_some() {
        &args.static_hostname
    } else {
        &mut_hostname
    };

    structs::ThemeData {
        last_exit_status: args.last_exit_status,
        datetime: date_time::date_time(),
        hostname: hostname.clone(),
        username: user_host::username(),
        python: python_status::python_info(),
        git: git_info.flatten(),
    }
}
