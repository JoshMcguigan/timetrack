use std::process::Child;
use std::process::Command;
use std::process::Stdio;

fn tracker_proc() -> Child {
    Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--track")
        .spawn()
        .expect("failed to execute child")
}

fn clear_proc() -> Child {
    Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--clear")
        .spawn()
        .expect("failed to execute child")
}

fn calc_proc() -> Child {
    Command::new("cargo")
        .arg("run")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child")
}

fn clear_and_verify() {
    let mut clear = clear_proc(); // clear logs
    clear.wait().unwrap();

    let calc = calc_proc();

    let output = calc
        .wait_with_output()
        .expect("failed to wait on child");

    let output_text = String::from_utf8_lossy(output.stdout.as_ref());

    assert!(output_text.contains("{}")); // ensure logs have been cleared
}

fn create_filesystem_noise(){
    let mut dummy_proc = calc_proc(); // dummy process creates traffic on the file system
    dummy_proc.wait().unwrap();
}

#[test]
#[ignore] // WARNING: this test clears all timetrack history
fn integration() {
    clear_and_verify();

    let mut tracker = tracker_proc(); // start watching file system

    create_filesystem_noise();

    let calc = calc_proc();

    let output = calc
        .wait_with_output()
        .expect("failed to wait on child");

    tracker.kill().expect("command wasn't running");

    let output_text = String::from_utf8_lossy(output.stdout.as_ref());

    assert!(output_text.contains("timetrack"));

    clear_and_verify();
}
