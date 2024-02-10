use clap::Parser;
use error::MapLog;
use std::borrow::Cow;
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
    args::init_argument_parser();
    let args = args::Args::parse();

    error::setup_errors(args.error_output);
    let theme_data = theme_data(&args);
    let symbols = args.symbols();

    print!("{}", args.theme()(&theme_data, symbols));

    Ok(())
}

fn theme_data(args: &args::Args) -> structs::ThemeData {
    let mut mut_hostname: Option<String> = None;
    let mut git_info: Option<structs::GitOutputOptions> = None;

    let fast_hostname = args
        .static_hostname
        .as_ref()
        .map(Cow::from)
        .or_else(|| std::env::var("HOST").map(Cow::from).ok_or_log()) // zsh and tcsh
        .or_else(|| std::env::var("HOSTNAME").map(Cow::from).ok_or_log()) // bash
        .or_else(|| std::env::var("COMPUTERNAME").map(Cow::from).ok_or_log()); // windows

    let git_info_options = structs::GetGitInfoOptions {
        start_folder: &args.git_start_folder,
        reference_name: args.git_reference.as_deref().unwrap_or("HEAD"),
        include_submodules: args.git_include_submodules,
        include_untracked: !args.git_exclude_untracked,
        refresh_status: args.git_refresh_status,
        include_ahead_behind: !args.git_exclude_ahead_behind,
        include_workdir_stats: !args.git_exclude_workdir_stats,
    };

    if fast_hostname.is_none() || !args.disable_git {
        thread::scope(|s| {
            s.spawn(|| {
                if fast_hostname.is_none() {
                    mut_hostname = user_host::hostname();
                }
            });

            s.spawn(|| {
                if !args.disable_git {
                    git_info = git_utils::process_current_dir(&git_info_options).ok_or_log();
                }
            });
        });
    }

    let hostname: Option<String> = fast_hostname.map(|s| s.to_string()).or(mut_hostname);

    structs::ThemeData {
        last_exit_status: args.last_exit_status,
        datetime: date_time::date_time(),
        hostname,
        username: user_host::username(),
        python: python_status::python_info(),
        git: git_info,
    }
}
