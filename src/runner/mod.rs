use std::collections::HashMap;

use anyhow::{Error, Result};

use crate::core::utils::sys::OS;
use crate::core::{models::script_model::ScriptModel, packages_manager::PackagesManager};
use crate::output::print_md_debug;
pub mod shell;
mod step_runner;

pub fn run_script(
    automation_name: &String,
    script: &ScriptModel,
    env: OS,
    packages_manager: &mut PackagesManager,
    params: HashMap<String, String>,
) -> Result<()> {
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
        step_statuses.push((step.title.clone(), success));
    }

    print_md_debug(&format!(
        "\n\n\n## 🚀🚀🚀 {} completed 🚀🚀🚀",
        automation_name
    ));
    for (title, success) in &step_statuses {
        let status_str = if *success { "✅" } else { "❌" };
        print_md_debug(&format!("{} {}", status_str, title));
    }

    // if any status is false return false
    let any_failed = step_statuses.iter().any(|(_, success)| !success);
    if any_failed {
        return Err(Error::msg("Some steps failed"));
    }

    Ok(())
}
