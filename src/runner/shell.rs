use std::collections::HashMap;
use std::process::Command;
use std::{env, process};

use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use tracing::error;

use crate::core::utils::packages::get_all_paths;
use crate::core::utils::sys::OS;
use crate::core::{models::script_model::StepModel, packages_manager::PackagesManager};
use crate::output::print_md_debug;


pub fn interactive_shell(
    packages_manager: &mut PackagesManager,
    command: String,
) -> Result<bool, String> {
    let paths_to_add = get_all_paths(&packages_manager);

    let cmd = "set -e\n".to_string() + command.as_str();

    // Get the current PATH and add a directory to it
    let mut new_path = paths_to_add.join(":");
    if let Some(current_path) = env::var("PATH").ok() {
        new_path.push_str(&current_path);
    }

    // Detect the OS based on your package_manager.settings.os
    let binding = env::var("SHELL")
                .unwrap_or("/bin/sh".to_string());
    let (shell, cmd_arg) = match &packages_manager.settings.os {
        OS::Windows => ("cmd.exe", "/C"),
        _ => (
            &*binding,
            "-c",
        ),
    };

    let status = Command::new(&shell)
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