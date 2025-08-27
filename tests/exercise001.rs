use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_exercise001_output() {
    let mut cmd = Command::cargo_bin("exercise001").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Name : Alex Johnson"))
        .stdout(predicate::str::contains("DOB : July 14, 1985"))
        .stdout(predicate::str::contains("Mobile : 999-999-9999"));
}
