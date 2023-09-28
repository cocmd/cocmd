use anyhow::{bail, Result};
use cocmd::core::sources_manager::SourcesManager;
use termimad::MadSkin;

pub fn run_docs(
    sources_manager: &mut SourcesManager,
    specific_name: &String,
    raw_markdown: bool,
) -> Result<cocmd::CmdExit> {
    let _skin = MadSkin::default();

    // load source with the specific name

    let source = sources_manager
        .get_source(specific_name.clone())
        .unwrap_or_else(|| panic!("Can't get source {}", specific_name));

    if !source.is_legit_cocmd_source() {
        bail!("Source {} is not a legit cocmd source", specific_name);
    }

    source.print_doc(&sources_manager.settings, !raw_markdown, false);

    Ok(cocmd::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}
