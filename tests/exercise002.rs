use assert_cmd::Command;
use predicates::prelude::*;

/**
 * 2. Rust Program: Adding two numbers
 *
 *
 * Write a Rust program that accepts two numbers from the user, adds them together, and displays the result.
 *
 */
#[test]
fn test_exercise002_output() {
    let mut cmd = Command::cargo_bin("exercise002").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Name : Alex Johnson"))
        .stdout(predicate::str::contains("DOB : July 14, 1985"))
        .stdout(predicate::str::contains("Mobile : 999-999-9999"));
}
