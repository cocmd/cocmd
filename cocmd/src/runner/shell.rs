use std::collections::HashMap;
use std::io::{self, BufRead};
use std::process;
use std::process::Stdio;

use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use execute::shell;
use minijinja::{Environment, Value};
use tracing::error;

use crate::core::models::script_model::StepParamModel;
use crate::core::{models::script_model::StepModel, packages_manager::PackagesManager};
use crate::output::print_md;

pub fn interactive_shell(
    step: &StepModel,
    params: Vec<StepParamModel>,
    packages_manager: &mut PackagesManager,
) -> Result<bool, String> {
    let command = step.content.as_ref().unwrap();

    // replace all {...} occurences in the command with the values from the params
    // get the value of the param by param.name and the function settings.get_param
    // if param.save == true, call save_param with the param.name and the value

    let mut cmd = "set -e\n".to_string() + command.clone().as_str();

    let env = Environment::new();
    let mut params_map: HashMap<String, String> = HashMap::new();

    for param in params {
        let param_value = packages_manager.settings.get_param(&param.name);
        // if param_value is None, get it from STDIN with some nice prompt
        let param_value = match param_value {
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
        };

        params_map.insert(param.name.clone(), param_value.clone());

        if param.save {
            packages_manager
                .settings
                .save_param(&param.name, &param_value);
        }
    }

    let ctx = Value::from_serializable(&params_map);
    cmd = env.render_str(&cmd, ctx).unwrap();

    let mut command = shell(cmd);

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
        print_md(&"## ✅ Success".to_string());
        Ok(true)
    } else {
        print_md(&"## ❌ Failed (stderr):\n".to_string());
        Err("Shell command failed.".to_string())
    }
}
