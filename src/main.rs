mod error;
mod git_utils;
mod structs;
mod util;

type Result<T, E = error::Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let full_name = Some(String::from("hello world"));
    let short_name: Option<String> = full_name
        .as_ref()
        .map(|v| v.strip_prefix("refs/heads/").unwrap_or(v))
        .map(|v| String::from(v));

    let a = error::error_control(git_utils::process_current_dir(
        &structs::GetGitInfoOptions::new(),
    ))?;
    println!("{:?}", a);
    Ok(())
}
