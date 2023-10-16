use std::collections::HashMap;
use std::io::{self, BufRead};
use std::process::Stdio;
use std::{env, process};

use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use execute::shell;
use regex::Regex;
use tracing::error;

use crate::core::models::script_model::StepParamModel;
use crate::core::utils::packages::get_all_paths;
use crate::core::{models::script_model::StepModel, packages_manager::PackagesManager};
use crate::output::print_md_debug;

pub fn interactive_shell(
    step: &StepModel,
    step_params: Vec<StepParamModel>,
    packages_manager: &mut PackagesManager,
    params: HashMap<String, String>,
) -> Result<bool, String> {
    let paths_to_add = get_all_paths(&packages_manager);

    let command = step.content.as_ref().unwrap();

    // replace all {...} occurences in the command with the values from the params
    // get the value of the param by param.name and the function settings.get_param
    // if param.save == true, call save_param with the param.name and the value

    let mut cmd = "set -e\n".to_string() + command.clone().as_str();

    let mut params_map: HashMap<String, String> = HashMap::new();

    for param in step_params {
        // look for param.name in params
        // if found, use that value
        // if not found, look for it in settings
        // if not found, ask the user for it

        let param_value = params.get(&param.name);

        let param_value = match param_value {
            Some(value) => value.clone(),
            None => {
                let param_value = packages_manager.settings.get_param(&param.name);
                match param_value {
                    Some(value) => value,
                    None => {
                        let prompt = format!("Enter value for parameter '{}'", param.name);
                        let param_value = dialoguer::Input::with_theme(&ColorfulTheme::default())
                            .with_prompt(&prompt)
                            .interact_text()
                            .unwrap_or_else(|_e| {
                                error!("No value entered for parameter '{}'.", param.name);
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
                .save_param(&param.name, &param_value);
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

    let mut command = shell(cmd);

    // Get the current PATH and add a directory to it
    let mut new_path = paths_to_add.join(":");
    if let Some(current_path) = env::var("PATH").ok() {
        new_path.push_str(&current_path);
    }
    command.env("PATH", new_path);

    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    command.stdin(Stdio::piped());

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
