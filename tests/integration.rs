extern crate timetrack;
use timetrack::track;
use timetrack::calc;

#[test]
fn test_add() {
    use std::process::{Command, Stdio};

    let mut tracker = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--track")
        .spawn()
        .expect("failed to execute child");

    let child = Command::new("cargo")
        .arg("run")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    let output = child
        .wait_with_output()
        .expect("failed to wait on child");

    tracker.kill().expect("command wasn't running");

    assert!(String::from_utf8(output.stdout).unwrap().contains("timetrack"));
}
