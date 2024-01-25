//! Integration / end2end / execution tests example

use std::env;
use std::ops::Not;
use std::path::Path;
use std::process::Command;

pub fn bin_path() -> &'static Path {
    const BIN_PATH: &'static str = env!("CARGO_BIN_EXE_upfind");
    Path::new(BIN_PATH)
}

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    let result = Command::new(bin_path())
        .arg("help")
        .current_dir(env::current_dir()?)
        .output()?;

    assert!(result.status.success()); // check exit code

    // To get prints to stdout run tests with `--nocapture`,
    // e.g.: `cargo test -- --nocapture`.
    println!("out: {}", std::str::from_utf8(&result.stdout)?);
    println!("err: {}", std::str::from_utf8(&result.stderr)?);

    // Or more accurate way:
    result
        .stdout
        .is_empty()
        .not()
        .then(|| std::str::from_utf8(&result.stdout))
        .map_or_else(
            || println!("stdout is empty"),
            |out| {
                let text = out.expect("stdout is valid utf-8");
                println!("stdout: {text}");
            },
        );

    // TODO: check output

    Ok(())
}
