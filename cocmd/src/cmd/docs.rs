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
use termimad;
use tracing::{error, info};

pub fn run_docs(
    sources_manager: &mut SourcesManager,
    specific_name: Option<String>,
) -> Result<cocmd::CmdExit> {
    Ok(cocmd::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}
