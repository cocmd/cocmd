use anyhow::{bail, Result};
use cocmd::core::packages_manager::PackagesManager;
use termimad::MadSkin;

pub fn run_docs(
    packages_manager: &mut PackagesManager,
    specific_name: &String,
    raw_markdown: bool,
) -> Result<cocmd::CmdExit> {
    let _skin = MadSkin::default();

    // load package with the specific name

    let package = packages_manager
        .get_package(specific_name.clone())
        .unwrap_or_else(|| panic!("Can't get package {}", specific_name));

    if !package.is_legit_cocmd_package() {
        bail!("Package {} is not a legit cocmd package", specific_name);
    }

    package.print_doc(&packages_manager.settings, !raw_markdown, false);

    Ok(cocmd::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}
