use anyhow::Result;
use cocmd::core::sources_manager::SourcesManager;
use termimad::MadSkin;

pub fn run_docs(
    _sources_manager: &mut SourcesManager,
    _specific_name: &String,
) -> Result<cocmd::CmdExit> {
    let skin = MadSkin::default();

    // load source with the specific name
    let source = _sources_manager.sources.get(_specific_name);
    if let None = source {
        return Ok(cocmd::CmdExit {
            code: exitcode::NOINPUT,
            message: Some(format!("Source with name {} not found", _specific_name)),
        });
    }

    Ok(cocmd::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}
