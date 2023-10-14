use std::collections::HashMap;
use std::process::Command;

use dialoguer::Confirm;
use tracing::error;

use super::shell::interactive_shell;
use crate::core::models::script_model::StepParamModel;
use crate::core::utils::sys::OS;
use crate::core::{
    models::script_model::{StepModel, StepRunnerType},
    packages_manager::PackagesManager,
};
use crate::output::print_md;

pub fn handle_step(
    step: &StepModel,
    env: OS,
    script_params: Option<Vec<StepParamModel>>,
    packages_manager: &mut PackagesManager,
    params: HashMap<String, String>,
) -> bool {
    let content = step.content.as_ref().unwrap().as_str();
    let script_params = step.get_params(script_params);

    print_md(&format!("# running shell step - {}", &step.title));
    if let Some(msg) = step.approval_message.clone() {
        if !Confirm::new().with_prompt(msg).interact().unwrap() {
            return false;
        }
    }

    match &step.runner {
        StepRunnerType::SHELL => {
            if let Err(_err) =
                interactive_shell(step, script_params.clone(), packages_manager, params)
            {
                return false;
            }
        }
        StepRunnerType::COCMD => {
            print_md(&format!("# running cocmd step - {}", &step.title));

            let provider_name = content.split('.').next().unwrap();

            let available_automations = packages_manager.automations();
            if !available_automations.contains_key(content) {
                if !Confirm::new()
                    .with_prompt(format!(
                        "Cocmd Package {} not found. Download?",
                        &provider_name
                    ))
                    .interact()
                    .unwrap()
                {
                    return false;
                }

                // ask the user if he wants to download the package. get yes/no approval
                // if yes, download the package
                if let Err(_err) = interactive_shell(
                    &StepModel {
                        content: Some(format!("cocmd install {}", &provider_name)),
                        ..step.clone()
                    },
                    script_params.clone(),
                    packages_manager,
                    HashMap::new(),
                ) {
                    return false;
                }
            }
            if let Err(_err) = interactive_shell(
                &StepModel {
                    content: Some(format!("cocmd run {}", &content)),
                    ..step.clone()
                },
                script_params.clone(),
                packages_manager,
                HashMap::new(),
            ) {
                return false;
            }
        }
        StepRunnerType::MARKDOWN => {
            // Print Markdown content
            print_md(&content.to_string());
        }
        StepRunnerType::PYTHON => {
            // Execute Python script
            let output = Command::new("python")
                .arg("-c")
                .arg(content)
                .output()
                .expect("Failed to execute Python script.");

            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
        StepRunnerType::LINK => {
            // Open URL in default browser (platform-specific)
            match env {
                OS::Windows => {
                    Command::new("cmd")
                        .arg("/C")
                        .arg("start")
                        .arg(content)
                        .spawn()
                        .expect("Failed to open link in the default browser.");
                }
                OS::Linux => {
                    Command::new("xdg-open")
                        .arg(content)
                        .spawn()
                        .expect("Failed to open link in the default browser.");
                }
                OS::MacOS => {
                    Command::new("open")
                        .arg(content)
                        .spawn()
                        .expect("Failed to open link in the default browser.");
                }
                _ => {
                    error!("unable to open link in the default browser.")
                }
            }
        }
    }
    true
}
