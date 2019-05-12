use log::{debug, log};
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

pub fn contains_file_which_would_not_be_ignored<I, S, P>(dir: P, paths: I) -> bool
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    P: AsRef<Path>,
{
    match Command::new("git")
        .current_dir(dir)
        .args(&["check-ignore", "-v", "--no-index", "-n"])
        .args(paths)
        .output()
    {
        Ok(output) => {
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .any(|line| line.contains("::"))
                || String::from_utf8_lossy(&output.stderr)
                    .lines()
                    .any(|line| line.starts_with("fatal: Not a git repository"))
        }
        Err(err) => {
            // this could be running git outside of a git repo (file change detected outside of a git repo)
            // or if the user doesn't have git installed
            debug!("{}", err);
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn contains() {
        let file_paths = vec!["target", "not-ignored"];
        assert!(contains_file_which_would_not_be_ignored(
            env::current_dir().unwrap(),
            file_paths
        ));
    }

    #[test]
    fn does_not_contains() {
        let file_paths = vec!["target"];
        assert!(!contains_file_which_would_not_be_ignored(
            env::current_dir().unwrap(),
            file_paths
        ));
    }
}
