use anyhow::{bail, Result};
use cocmd::core::sources_manager::SourcesManager;
use termimad::MadSkin;

pub fn run_docs(
    sources_manager: &mut SourcesManager,
    specific_name: &String,
    raw_markdown: bool,
) -> Result<cocmd::CmdExit> {
    let skin = MadSkin::default();

    // load source with the specific name
    let source = sources_manager.sources.get(specific_name);
    if let None = source {
        return Ok(cocmd::CmdExit {
            code: exitcode::NOINPUT,
            message: Some(format!("Source with name {} not found", specific_name)),
        });
    }
    let source = source.unwrap();

    if !source.is_legit_cocmd_source() {
        bail!("Source {} is not a legit cocmd source", specific_name);
    }

    source.print_doc(&sources_manager.settings, !raw_markdown, false);

    Ok(cocmd::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}
