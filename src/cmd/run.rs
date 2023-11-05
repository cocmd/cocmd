use std::process;

use anyhow::{Error, Result};
use dialoguer::{theme::ColorfulTheme, Select};
use log::error;

use crate::core::packages_manager::{self, PackagesManager};
use crate::core::utils::cmd::parse_params;
use crate::package_provider::get_provider;
use crate::runner::run_script;

pub fn run_automation(
    packages_manager: &mut PackagesManager,
    specific_name: Option<String>,
    params: Option<Vec<String>>,
    from: Option<String>,
) -> Result<()> {
    let mut packages_manager = packages_manager;
    let mut limit_path = Option::<String>::None;

    if let Some(from) = from {
        let provider =
            get_provider(&from.to_string(), &packages_manager.settings.runtime_dir).unwrap();
        if !provider.is_exists_locally() {
            provider.download().unwrap();
        }

        limit_path = provider.local_path().to_str().map(|s| s.to_string());
    }

    let mut new_packages_manager;

    if limit_path.is_some() {
        new_packages_manager = packages_manager::PackagesManager::new(
            packages_manager.settings.clone(),
            limit_path.clone(),
        );
        packages_manager = &mut new_packages_manager;
    }

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
