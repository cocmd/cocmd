use anyhow::{bail, Error, Result};
use log::error;

use crate::core::{
    packages_manager::PackagesManager,
    utils::packages::{get_package_name_from_uri, get_playbook_name_from_uri},
};

pub fn run_docs(
    packages_manager: &mut PackagesManager,
    uri: &String,
    raw_markdown: bool,
    output_file: Option<String>,
) -> Result<()> {
    // load package with the specific name

    let package_name = get_package_name_from_uri(uri);
    let playbook_name = get_playbook_name_from_uri(uri);

    let package = packages_manager
        .get_package(package_name.clone())
        .map_or_else(
            || {
                error!("Can't get package {}", uri);
                Err(Error::msg("can't get package"))
            },
            |pkg| Ok(pkg),
        )?;

    if !package.is_legit_cocmd_package() {
        bail!("Package {} is not a legit cocmd package", uri);
    }

    if let Some(playbook_name) = playbook_name {
        let playbook = package.get_playbook(playbook_name.clone()).map_or_else(
            || {
                error!("Can't get playbook {}", playbook_name);
                Err(Error::msg("can't get playbook"))
            },
            |pb| Ok(pb),
        )?;

        playbook.print_doc(
            &packages_manager.settings,
            !raw_markdown,
            false,
            output_file,
            uri.clone(),
        );

        return Ok(());
    }

    package.print_doc(
        &packages_manager.settings,
        !raw_markdown,
        false,
        output_file,
    );

    Ok(())
}
