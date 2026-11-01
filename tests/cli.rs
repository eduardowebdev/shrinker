use std::error::Error;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn help_lists_subcommands() -> Result<(), Box<dyn Error>> {
    Command::cargo_bin("shrinker")?
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Lossless image compression"))
        .stdout(predicate::str::contains("compress"))
        .stdout(predicate::str::contains("info"));
    Ok(())
}

#[test]
fn info_on_missing_file_errors() -> Result<(), Box<dyn Error>> {
    Command::cargo_bin("shrinker")?
        .args(["info", "/this/path/does/not/exist.png"])
        .assert()
        .failure();
    Ok(())
}

#[test]
fn compress_requires_inputs() -> Result<(), Box<dyn Error>> {
    Command::cargo_bin("shrinker")?
        .arg("compress")
        .assert()
        .failure();
    Ok(())
}
