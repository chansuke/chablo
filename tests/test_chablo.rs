use std::error::Error;
use std::process::Command;

use assert_cmd::{assert::OutputAssertExt, prelude::CommandCargoExt};
use predicates::prelude::*;

#[test]
fn test_chablo_ok() -> Result<(), Box<dyn Error>> {
    let mut bin = bin();
    bin.arg("build");
    bin.assert().success();

    Ok(())
}

#[test]
fn test_chablo_error() -> Result<(), Box<dyn Error>> {
    let mut bin = bin();
    bin.arg("unknown_command");

    let msg = "error: unrecognized subcommand 'unknown_command'\n\nUsage: chablo <COMMAND>\n\nFor more information, try '--help'.";

    bin.assert().failure().stderr(predicate::str::contains(msg));
    Ok(())
}

fn bin() -> Command {
    Command::cargo_bin("chablo").unwrap()
}
