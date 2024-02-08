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
    args::init_argument_parser();
    let args = args::Args::parse();

    error::setup_errors(args.error_output);
    let theme_data = theme_data(&args);
    let symbols = args.symbols();

    print!("{}", args.theme()(&theme_data, symbols));

    Ok(())
}

fn theme_data(args: &args::Args) -> structs::ThemeData {
    let mut mut_hostname: Option<Option<String>> = Some(None);
    let mut git_info: Option<Option<structs::GitOutputOptions>> = Some(None); // TODO: Option in
                                                                              // Option may be just
                                                                              // option

    let fast_hostname = args
        .static_hostname
        .clone() // TODO: Clone
        .or_else(|| std::env::var("HOST").ok_or_log()) // zsh and tcsh
        .or_else(|| std::env::var("HOSTNAME").ok_or_log()) // bash
        .or_else(|| std::env::var("COMPUTERNAME").ok_or_log()); // windows

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
                    let _ = mut_hostname.insert(user_host::hostname());
                }
            });

            s.spawn(|| {
                if !args.disable_git {
                    let _ = git_info
                        .insert(git_utils::process_current_dir(&git_info_options).ok_or_log());
                }
            });
        });
    }
    let hostname = fast_hostname
        .as_ref()
        .or(mut_hostname.flatten().as_ref())
        .cloned();
    //        &args.static_hostname
    //   } else {
    //       &mut_hostname
    //   };

    structs::ThemeData {
        last_exit_status: args.last_exit_status,
        datetime: date_time::date_time(),
        hostname,
        username: user_host::username(),
        python: python_status::python_info(),
        git: git_info.flatten(),
    }
}
