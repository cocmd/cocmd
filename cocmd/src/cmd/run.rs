use std::io::{self, BufRead};
use std::process;
use std::process::{Command, Stdio};

use anyhow::Result;
use cocmd::core::{
    models::script_model::{ScriptModel, StepModel, StepRunnerType},
    sources_manager::SourcesManager,
};
use cocmd::utils::sys::OS;
use dialoguer::{theme::ColorfulTheme, Select};
use execute::{shell, Execute};
use inline_colorization::*;
use termimad::{self, MadSkin};
use tracing::{error, info};

pub fn run_automation(
    sources_manager: &mut SourcesManager,
    specific_name: Option<String>,
) -> Result<cocmd::CmdExit> {
    let available_automations = sources_manager.automations();

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
        // let output = ScriptRunner::run(script, &settings.os, &script_args, &settings, auto_yes);
        // info!("{:?}", script);
        // let output = script.content;
        handle_script(
            automation.content.as_ref().unwrap(),
            sources_manager.settings.os,
        );
        // info!("[blue] Script executed:");
        // for line in output {
        //     info!(" - {}", line);
        // }

        info!("Script {} completed", automation.name);
    } else {
        error!("I don't know this script");
    }

    Ok(cocmd::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}

fn interactive_shell(step: &StepModel, skin: &mut MadSkin) -> Result<bool, String> {
    let mut command = shell(step.content.as_ref().unwrap());

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
        skin.print_text("## ✅ Success");
        Ok(true)
    } else {
        skin.print_text("## ❌ Failed (stderr):\n");
        Err("Shell command failed.".to_string())
    }
}

fn handle_step(step: &StepModel, env: OS, skin: &mut MadSkin) -> bool {
    let content = step.content.as_ref().unwrap().as_str();
    match &step.runner {
        StepRunnerType::SHELL => {
            skin.print_text(&format!("# running shell step - {}", &step.title));
            if let Err(err) = interactive_shell(step, skin) {
                return false;
            }
        }
        StepRunnerType::COCMD => {
            skin.print_text(&format!("# running cocmd step - {}", &step.title));
            if let Err(err) = interactive_shell(
                &StepModel {
                    content: Some(format!("cocmd run {}", &content)),
                    ..step.clone()
                },
                skin,
            ) {
                return false;
            }
        }
        StepRunnerType::MARKDOWN => {
            // Print Markdown content
            skin.print_text(content);
        }
        StepRunnerType::PYTHON => {
            skin.print_text(&format!("# running python step - {}", &step.title));
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
                OS::Other => todo!(),
                OS::ANY => todo!(),
            }
        }
    }
    return true;
}

fn handle_script(script: &ScriptModel, env: OS) {
    let mut skin: MadSkin = MadSkin::default();
    let mut step_statuses = Vec::new();

    for step in &script.steps {
        let success = handle_step(step, env, &mut skin);
        step_statuses.push((step.title.clone(), success));
    }

    println!();
    println!("# Automation completed. Summary:");
    for (title, success) in &step_statuses {
        let status_str = if *success { "✅" } else { "❌" };
        println!("{} {}", status_str, title);
    }

    println!(); // Add a newline after the summary
}
