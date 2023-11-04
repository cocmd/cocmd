use std::path::Path;

use anyhow::{bail, Result};
use dialoguer::{theme::ColorfulTheme, Select};
use log::{error, info};

use crate::core::packages_manager::PackagesManager;
// add to bashrc or zshrc the following line if not exists
// eval "$(cocmd profile-loader)"
// output to stdout with tracing::info what you did
pub fn run_setup(_packages_manager: &mut PackagesManager, shell: Option<String>) -> Result<()> {
    let mut shell = shell.unwrap_or_else(|| {
        if let Some(shell) = std::env::var_os("SHELL") {
            shell.to_string_lossy().to_string()
        } else {
            // using termimad, select shell from list of shells
            let shell_choices: Vec<&str> = vec!["bash", "zsh"];
            let selected_shell = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("What shell to setup?")
                .items(&shell_choices)
                .default(0) // Set a default selection if needed
                .interact_opt()
                .unwrap_or_else(|_e| {
                    error!("No shell selected.");
                    panic!("No shell selected.");
                });
            shell_choices[selected_shell.unwrap()].to_string()
        }
    });
    shell = shell.trim().to_lowercase();

    // TODO: extract the shell from SHELL env which can be also be in some path */zsh or */bash and such

    let shell = match shell.as_str() {
        "bash" => "bash",
        "/bin/bash" => "bash",
        "zsh" => "zsh",
        "/bin/zsh" => "zsh",
        _ => {
            bail!("Unsupported shell: {}. Supported shells: bash, zsh", shell);
        }
    };
    let profile_loader = r#"
# cocmd profile-loader
eval "$(cocmd profile-loader)"
"#
    .to_string();
    let profile_path = match shell {
        "bash" => {
            let home = std::env::var("HOME").unwrap();
            format!("{}/.bashrc", home)
        }
        "zsh" => {
            let home = std::env::var("HOME").unwrap();
            format!("{}/.zshrc", home)
        }
        _ => unreachable!(),
    };
    let mut profile = match std::fs::read_to_string(&profile_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "Failed to read the file at {}: {}",
                Path::new(&profile_path).display(),
                e
            );
            return Err(anyhow::Error::new(e));
        }
    };

    // check if profile_loader is already in profile
    if profile.contains("cocmd profile-loader") {
        info!("Already added profile-loader to {}", profile_path);
        return Ok(());
    }

    profile.push_str(&profile_loader);
    if let Err(e) = std::fs::write(&profile_path, profile) {
        bail!("Failed to write to {}: {}", profile_path, e);
    }
    info!("Added profile-loader to {}", profile_path);
    Ok(())
}
