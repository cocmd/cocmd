use std::process;

use anyhow::{Error, Result};
use dialoguer::{theme::ColorfulTheme, Select};
use tracing::error;

use crate::core::packages_manager::PackagesManager;
use crate::core::utils::cmd::parse_params;
use crate::runner::run_script;
pub fn run_automation(
    packages_manager: &mut PackagesManager,
    specific_name: Option<String>,
    params: Option<Vec<String>>,
) -> Result<()> {
    let available_automations = packages_manager.automations();

    let selected_name = match specific_name {
        Some(name) => name,
        None => {
            let script_choices: Vec<&String> = available_automations.keys().collect();
            let selected_script = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("What script to run?")
                .items(&script_choices)
                .default(0) // Set a default selection if needed
                .interact_opt()
                .unwrap_or_else(|_e| {
                    error!("No script selected.");
                    process::exit(1)
                });

            script_choices[selected_script.unwrap()].to_string()
        }
    };

    if let Some(automation) = available_automations.get(&selected_name) {
        return run_script(
            &selected_name,
            automation.content.as_ref().unwrap(),
            packages_manager.settings.os,
            packages_manager,
            parse_params(params),
        );
    }

    // return Err(Error("I don't know this script"));
    // return an error that the script is not found
    Err(Error::msg("I don't know this script"))
}
