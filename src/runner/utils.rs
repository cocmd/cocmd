#![allow(clippy::non_minimal_cfg)]

use std::process::Command;

pub fn check_installed(cmd: &str) -> bool {
    let output = Command::new(cmd).output();

    if output.is_err() {
        return false;
    }

    output.unwrap().status.success()
}

// write a test for check_installed
// two case - a command that exists for sure and one that doesn't

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[test]
    fn test_check_installed() {
        assert!(check_installed("ls"));
        assert!(!check_installed("dummy_command"));
    }

    #[cfg(any(target_os = "windows"))]
    #[test]
    fn test_check_installed() {
        assert_eq!(check_installed("dir"), true);
        assert_eq!(check_installed("dummy_command"), false);
    }
}
