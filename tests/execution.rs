//! Integration / end2end / execution tests example

use std::ops::Not;
use std::process::Command;

mod common;
use common::*;

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    // create environment for this teat
    let path = {
        // TODO: create inner files
        // std::fs::create_dir(&p.join(".git"))?;
        tmp_for("--help")?
    };

    let result = Command::new(bin_path())
        .arg("--help")
        //   .current_dir(env::current_dir()?)
        .current_dir(path)
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
