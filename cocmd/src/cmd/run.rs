use std::process;

use anyhow::Result;
use cocmd_core::packages_manager::PackagesManager;
use cocmd_log::{cocmd_error, tracing};
use cocmd_runner::run_script;
use dialoguer::{theme::ColorfulTheme, Select};

use super::CmdExit;
pub fn run_automation(
    packages_manager: &mut PackagesManager,
    specific_name: Option<String>,
) -> Result<CmdExit> {
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
                    cocmd_error!("No script selected.");
                    process::exit(1)
                });

            script_choices[selected_script.unwrap()].to_string()
        }
    };

    if let Some(automation) = available_automations.get(&selected_name) {
        // let output = ScriptRunner::run(script, &settings.os, &script_args, &settings, auto_yes);
        // info!("{:?}", script);
        // let output = script.content;
        run_script(
            &selected_name,
            automation.content.as_ref().unwrap(),
            packages_manager.settings.os,
            packages_manager,
        );
        // info!("[blue] Script executed:");
        // for line in output {
        //     info!(" - {}", line);
        // }

        // info!("Script {} completed", automation.name);
    } else {
        cocmd_error!("I don't know this script");
    }

    Ok(CmdExit {
        code: exitcode::OK,
        message: None,
    })
}
