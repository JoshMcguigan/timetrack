use std::process::Child;
use std::process::Command;
use std::process::Stdio;
use std::time;
use std::thread;
use std::fs::OpenOptions;
use std::fs;
use std::io::Write;
extern crate timetrack;
use timetrack::config::get_config;

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
    let test_file_path = get_config().track_paths.get(0).unwrap().to_owned() + "/timetrack/__integration_test__";

    {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(&test_file_path).unwrap();
        write!(file, "testing");
    }

    let sleep_duration = time::Duration::from_millis(500);
    thread::sleep(sleep_duration);

    fs::remove_file(&test_file_path).unwrap();

    // sleep to ensure file system noise was captured
    let sleep_duration = time::Duration::from_millis(2500);
    thread::sleep(sleep_duration);
}

#[test]
//#[ignore] // WARNING: this test clears all timetrack history, TODO perhaps move the existing history file before testing then move it back?
fn integration() {
    // Ensure there are no instances of TimeTrack running while this test is running

    clear_and_verify();

    let mut tracker = tracker_proc(); // start watching file system

    create_filesystem_noise();

    tracker.kill().expect("command wasn't running");

    let calc = calc_proc();

    let output = calc
        .wait_with_output()
        .expect("failed to wait on child");

    let output_text = String::from_utf8_lossy(output.stdout.as_ref());

    assert!(output_text.contains("timetrack"));

    clear_and_verify();
}
