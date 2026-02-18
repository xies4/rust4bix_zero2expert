use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn prints_dash_by_default() {
    let mut cmd = Command::cargo_bin("gc").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("INPUT\t-"));
}

#[test]
fn prints_path_when_provided() {
    let mut cmd = Command::cargo_bin("gc").unwrap();
    cmd.arg("file.fa");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("INPUT\tfile.fa"));
}
