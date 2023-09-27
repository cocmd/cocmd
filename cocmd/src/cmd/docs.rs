use anyhow::Result;
use cocmd::core::sources_manager::SourcesManager;

pub fn run_docs(
    _sources_manager: &mut SourcesManager,
    _specific_name: Option<String>,
) -> Result<cocmd::CmdExit> {
    Ok(cocmd::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}
