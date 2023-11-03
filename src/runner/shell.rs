use std::env;
use std::process::Command;

use anyhow::Result;

use crate::core::packages_manager::PackagesManager;
use crate::core::utils::packages::get_all_paths;
use crate::core::utils::sys::OS;
use crate::output::print_md_debug;

pub fn interactive_shell(
                packages_manager: &mut PackagesManager,

    command: String,
) -> Result<bool, String> {
    let paths_to_add = get_all_paths(packages_manager);

    let cmd = "set -e\n".to_string() + command.as_str();

    // Get the current PATH and add a directory to it
    let mut new_path = paths_to_add.join(":");
    if let Ok(current_path) = env::var("PATH") {
        new_path.push_str(&current_path);
    }

    // Detect the OS based on your package_manager.settings.os
    let binding = env::var("SHELL").unwrap_or("/bin/sh".to_string());
    let (shell, cmd_arg) = match &packages_manager.settings.os {
        OS::Windows => ("cmd.exe", "/C"),
        _ => (&*binding, "-c"),
    };

    let status = Command::new(shell)
        .arg(cmd_arg)
        .arg(cmd)
        .env("PATH", new_path)
        .status()
        .map_err(|e| format!("Failed to start shell command: {}", e))?;

    if status.success() {
        Ok(true)
    } else {
        print_md_debug(&"## ❌ Failed (stderr):\n".to_string());
        Err("Shell command failed.".to_string())
    }
}

#[cfg(target_os = "windows")]
mod windows_tests {
    use super::*;
    use crate::core::{models::settings::Settings, packages_manager::PackagesManager};

    #[test]
    fn test_interactive_shell_windows() {
        let settings = Settings::new(None, None);
        let mut packages_manager = PackagesManager::new(settings);

        // Call the interactive_shell function with test data for Windows
        let command = "echo Hello, Windows".to_string();
        let result = interactive_shell(&mut packages_manager, command);

        // Assert that the result is as expected
        assert_eq!(result, Ok(true));
    }
}

#[cfg(target_os = "linux")]
mod linux_tests {
    use super::*;
    use crate::core::{models::settings::Settings, packages_manager::PackagesManager};

    #[test]
    fn test_interactive_shell_linux() {
        let settings = Settings::new(None, None);
        let mut packages_manager = PackagesManager::new(settings);

        // Call the interactive_shell function with test data for Linux
        let command = "echo Hello, Linux".to_string();
        let result = interactive_shell(&mut packages_manager, command);

        // Assert that the result is as expected
        assert_eq!(result, Ok(true));
    }
}

#[cfg(target_os = "macos")]
mod macos_tests {

    #[test]
    fn test_interactive_shell_macos() {
        let settings = Settings::new(None, None);
        let mut packages_manager = PackagesManager::new(settings);

        // Call the interactive_shell function with test data for macOS
        let command = "echo Hello, macOS".to_string();
        let result = interactive_shell(&mut packages_manager, command);

        // Assert that the result is as expected
        assert_eq!(result, Ok(true));
    }
}
