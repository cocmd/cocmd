use std::collections::HashMap;
use std::io::{self, BufRead};
use std::process::Stdio;
use std::{env, process};

use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use execute::shell;
use tracing::error;

use crate::core::utils::packages::get_all_paths;
use crate::core::{models::script_model::StepModel, packages_manager::PackagesManager};
use crate::output::print_md_debug;

pub fn interactive_shell(
    packages_manager: &mut PackagesManager,
    command: String,
) -> Result<bool, String> {
    let paths_to_add = get_all_paths(&packages_manager);

    let mut cmd = "set -e\n".to_string() + command.clone().as_str();

    let mut command = shell(cmd);

    // Get the current PATH and add a directory to it
    let mut new_path = paths_to_add.join(":");
    if let Some(current_path) = env::var("PATH").ok() {
        new_path.push_str(&current_path);
    }
    command.env("PATH", new_path);

    // Ensure stdin is connected to the terminal
    command.stdin(Stdio::inherit());

    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    // command.stdin(Stdio::piped());

    let mut child = command
        .spawn()
        .map_err(|e| format!("Failed to start shell command: {}", e))?;

    // Create separate threads to read and print stdout and stderr
    let stdout = child.stdout.take().expect("Failed to open stdout");
    let stderr = child.stderr.take().expect("Failed to open stderr");

    let stdout_thread = std::thread::spawn(move || {
        let stdout_reader = io::BufReader::new(stdout);
        for line in stdout_reader.lines() {
            if let Ok(line) = line {
                println!("{}", line); // You can customize the output format here
            }
        }
    });

    let stderr_thread = std::thread::spawn(move || {
        let stderr_reader = io::BufReader::new(stderr);
        for line in stderr_reader.lines() {
            if let Ok(line) = line {
                eprintln!("{}", line); // You can customize the output format here
            }
        }
    });

    // Wait for the command to finish and get its exit status
    let status = child
        .wait()
        .map_err(|e| format!("Failed to wait for shell command: {}", e))?;

    let success = status.success();

    // Wait for the stdout and stderr threads to finish
    stdout_thread.join().expect("stdout thread panicked");
    stderr_thread.join().expect("stderr thread panicked");

    if success {
        // print_md_debug(&"## ✅ Success".to_string());
        Ok(true)
    } else {
        print_md_debug(&"## ❌ Failed (stderr):\n".to_string());
        Err("Shell command failed.".to_string())
    }
}
