use directories::UserDirs;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use TimeTracker;

#[allow(dead_code)]
fn not_supported() {
    println!("Schedule configuration is not supported on your operating system");
}

fn get_plist_file_contents() -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
    <dict>
        <key>Label</key>
        <string>rust.cargo.timetrack</string>
        <key>ProgramArguments</key>
        <array>
            <string>{}/.cargo/bin/timetrack</string>
            <string>track</string>
        </array>
        <key>RunAtLoad</key>
        <true/>
    </dict>
</plist>
"#,
        UserDirs::new().unwrap().home_dir().to_string_lossy()
    ).to_string()
}

fn get_plist_file_path() -> PathBuf {
    UserDirs::new()
        .unwrap()
        .home_dir()
        .join("Library/LaunchAgents/rust.cargo.timetrack.plist")
}

impl<'a> TimeTracker<'a> {
    #[cfg(target_os = "macos")]
    pub fn schedule(&self) {
        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(get_plist_file_path())
            .expect("Unable to open or create plist file");

        write!(&mut f, "{}", get_plist_file_contents())
            .expect("Failed to schedule TimeTrack");
        println!("TimeTrack scheduled. Logout/login to start tracking.");
    }

    #[cfg(not(target_os = "macos"))]
    pub fn schedule(&self) {
        not_supported();
    }

    #[cfg(target_os = "macos")]
    pub fn unschedule(&self) {
        match fs::remove_file(get_plist_file_path()) {
            Ok(_) => println!("TimeTrack schedule removed."),
            Err(_) => println!("Failed to remove TimeTrack schedule."),
        }
    }

    #[cfg(not(target_os = "macos"))]
    pub fn unschedule(&self) {
        not_supported();
    }
}
