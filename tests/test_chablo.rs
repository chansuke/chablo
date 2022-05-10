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
    bin.arg("buildddd_");

    let msg = if cfg!(windows) {
        "The subcommand \'buildddd_\' wasn\'t recognized\n\n\tDid you mean \'build\'?\n\nIf you believe you received this message in error, try re-running with \'chablo.exe -- buildddd_\'\n\nUSAGE:\n    chablo.exe <SUBCOMMAND>\n\nFor more information try --help\n"
    } else {
        "The subcommand \'buildddd_\' wasn\'t recognized\n\n\tDid you mean \'build\'?\n\nIf you believe you received this message in error, try re-running with \'chablo -- buildddd_\'\n\nUSAGE:\n    chablo <SUBCOMMAND>\n\nFor more information try --help\n"
    };

    bin.assert().failure().stderr(predicate::str::contains(msg));
    Ok(())
}

fn bin() -> Command {
    Command::cargo_bin("chablo").unwrap()
}
