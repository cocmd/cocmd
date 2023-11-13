use std::collections::HashMap;
use std::process;
use std::process::Command;

use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
use log::error;
use regex::Regex;

use super::shell::interactive_shell;
use crate::core::models::script_model::StepParamModel;
use crate::core::utils::packages::get_package_name_from_uri;
use crate::core::utils::sys::OS;
use crate::core::{
    models::script_model::{StepModel, StepRunnerType},
    packages_manager::PackagesManager,
};
use crate::output::{print_md, print_md_debug};
use crate::runner::utils::check_installed;

pub fn handle_step(
    step: &StepModel,
    env: OS,
    script_params: Option<Vec<StepParamModel>>,
    packages_manager: &mut PackagesManager,
    params: HashMap<String, String>,
) -> bool {
    let mut content = step.content.as_ref().unwrap().as_str();
    let script_params = step.get_params(script_params);

    print_md_debug(&format!("## {}", &step.title));
    if let Some(msg) = step.approval_message.clone() {
        if !Confirm::new().with_prompt(msg).interact().unwrap() {
            return true;
        }
    }

    let binding = apply_params_to_content(
        content.to_string(),
        script_params.clone(),
        packages_manager,
        params.clone(),
    );
    content = &binding;

    match &step.runner {
        StepRunnerType::SHELL => {
            if let Err(_err) = interactive_shell(packages_manager, content.to_string()) {
                return false;
            }
        }
        StepRunnerType::COCMD => {
            let provider_name = get_package_name_from_uri(content);

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
                    packages_manager,
                    format!("cocmd --no-verbose install {}", &provider_name),
                ) {
                    return false;
                }
            }
            if let Err(_err) = interactive_shell(
                packages_manager,
                format!("cocmd --no-verbose run {}", &content),
            ) {
                return false;
            }
        }
        StepRunnerType::MARKDOWN => {
            // Print Markdown content
            print_md(content);
        }
        StepRunnerType::PYTHON => {
            // make sure that "python" is installed and reachable from the command line
            if !check_installed("python") {
                print_md_debug("## ❌ Python not installed\n");
                return false;
            }

            // Execute Python script
            let output = Command::new("python")
                .arg("-c")
                .arg(content)
                .output()
                .expect("Failed to execute Python script.");

            println!("stdout: \n{}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: \n{}", String::from_utf8_lossy(&output.stderr));

            let success = output.status.success();

            if success {
                // print_md_debug(&"## ✅ Success".to_string());
                return true;
            } else {
                print_md_debug("## ❌ Failed\n");
                return false;
            }
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

fn apply_params_to_content(
    cmd: String,
    step_params: Vec<StepParamModel>,
    packages_manager: &mut PackagesManager,
    params: HashMap<String, String>,
) -> String {
    let mut cmd = cmd;
    let mut params_map: HashMap<String, String> = HashMap::new();

    for param in step_params.clone() {
        // look for param.name in params
        // if found, use that value
        // if not found, look for it in settings
        // if not found, ask the user for it
        let param_name = param.name.clone();
        let param_value = params.get(param_name.as_str());

        let param_value = match param_value {
            Some(value) => value.clone(),
            None => {
                let param_value = packages_manager.settings.get_param(&param_name);
                match param_value {
                    Some(value) => value,
                    None => {
                        let prompt = format!("Enter value for parameter '{}'", param_name);
                        let param_value = dialoguer::Input::with_theme(&ColorfulTheme::default())
                            .with_prompt(&prompt)
                            .interact_text()
                            .unwrap_or_else(|_e| {
                                error!("No value entered for parameter '{}'.", param_name);
                                process::exit(1)
                            });
                        param_value
                    }
                }
            }
        };

        params_map.insert(param.name.clone(), param_value.clone());

        if param.save {
            packages_manager
                .settings
                .save_param(&param_name, &param_value);
        }
    }

    // like in jinja2 parameters templating (but without using any jinja2 lib)
    // replace in cmd ocorunces of {{\s*...\s*}} with the values from params_map. ignore spaces inside the brackets
    // use regular expression to find all matches and ignore the spaces inside the brackets
    // for each match, replace it with the value from params_map

    // Compile regex to match {{ param }}
    let re = Regex::new(r"\{\s*\{\s*([\w.]+)\s*\}\s*\}").unwrap();

    // Find all parameter matches
    for cap in re.captures_iter(cmd.clone().as_ref()) {
        // Extract the parameter name
        let param_name = &cap[1];

        // Get the parameter value from params_map
        if let Some(param_value) = params_map.get(param_name) {
            // Replace match with param value
            cmd = re.replace(&cmd, param_value).to_string();
        } else {
            // Param not found error
        }
    }
    cmd.to_string()
}
