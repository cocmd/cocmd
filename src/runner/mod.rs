use std::collections::HashMap;

use crate::core::utils::sys::OS;
use crate::core::{
    models::script_model::{ScriptModel, StepRunnerType},
    packages_manager::PackagesManager,
};
use crate::output::print_md;
mod shell;
mod step_runner;

pub fn run_script(
    automation_name: &String,
    script: &ScriptModel,
    env: OS,
    packages_manager: &mut PackagesManager,
    params: HashMap<String, String>,
) {
    let mut step_statuses = Vec::new();
    let script_params = script.params.clone();
    for step in &script.steps {
        let success = step_runner::handle_step(
            step,
            env,
            script_params.clone(),
            packages_manager,
            params.clone(),
        );
        // check if step runner is executable shell/cmd/python add it
        if step.runner == StepRunnerType::SHELL
            || step.runner == StepRunnerType::COCMD
            || step.runner == StepRunnerType::PYTHON
        {
            step_statuses.push((step.title.clone(), success));
        }
    }

    print_md(&format!(
        "\n\n\n## 🚀🚀🚀 {} completed 🚀🚀🚀",
        automation_name
    ));
    for (title, success) in &step_statuses {
        let status_str = if *success { "✅" } else { "❌" };
        print_md(&format!("{} {}", status_str, title));
    }
}
