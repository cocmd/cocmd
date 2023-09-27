use anyhow::Result;
use cocmd::core::sources_manager::SourcesManager;
use dialoguer::{theme::ColorfulTheme, Select};
use tracing;

// add to bashrc or zshrc the following line if not exists
// eval "$(cocmd profile-loader)"
// output to stdout with tracing::info what you did
pub fn run_setup(
    _sources_manager: &mut SourcesManager,
    shell: Option<String>,
) -> Result<cocmd::CmdExit> {
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
                    tracing::error!("No shell selected.");
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
            return Err(anyhow::anyhow!(
                "Unsupported shell: {}. Supported shells: bash, zsh",
                shell
            ))
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
    let mut profile = std::fs::read_to_string(&profile_path)?;

    // check if profile_loader is already in profile
    if profile.contains("cocmd profile-loader") {
        tracing::info!("Already added profile-loader to {}", profile_path);
        return Ok(cocmd::CmdExit {
            code: exitcode::OK,
            message: None,
        });
    }

    profile.push_str(&profile_loader);
    std::fs::write(&profile_path, profile)?;
    tracing::info!("Added profile-loader to {}", profile_path);
    Ok(cocmd::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}
