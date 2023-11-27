use std::path::Path;

use anyhow::{bail, Error, Result};
use log::error;

use crate::core::{
    consts::GEN_MESSAGE,
    packages_manager::PackagesManager,
    utils::{
        io::file_write,
        packages::{get_package_name_from_uri, get_playbook_name_from_uri},
    },
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

    // if output_file exists, delete it
    if let Some(output_file) = &output_file {
        if std::path::Path::new(output_file).exists() {
            std::fs::remove_file(output_file).map_err(|e| {
                error!("Failed to delete output file: {}", e);
                Error::msg("failed to delete output file")
            })?;
        }
    }

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
        let playbook_envs_map = package
            .get_playbook_envs_map(playbook_name.clone())
            .map_or_else(
                || {
                    error!("Can't get playbook {}", playbook_name);
                    Err(Error::msg("can't get playbook"))
                },
                |pb| Ok(pb),
            )?;

        if output_file.is_some() {
            let output_path = Path::new(output_file.as_ref().unwrap().as_str());
            file_write(&output_path, &format!("# {}\n\n\n", uri), false);

            // write table of contents for each os
            for (os, _) in playbook_envs_map.clone() {
                file_write(
                    &output_path,
                    &format!("- [{}](#{})\n", format!("{}", os), format!("{}", os)),
                    false,
                );
            }

            file_write(&output_path, "\n\n", false);
        }

        for (os, playbook) in playbook_envs_map {
            playbook.print_doc(
                &packages_manager.settings,
                !raw_markdown,
                false,
                output_file.clone(),
                format!("{}", os),
            );
        }

        write_signature_if_needed(&output_file);

        return Ok(());
    }

    package.print_doc(
        &packages_manager.settings,
        !raw_markdown,
        false,
        output_file.clone(),
    );

    write_signature_if_needed(&output_file);

    Ok(())
}

fn write_signature_if_needed(output_file: &Option<String>) -> Result<()> {
    if let Some(output_file) = output_file {
        file_write(Path::new(&output_file), GEN_MESSAGE, false).or_else(|e| {
            error!("Unable to write to file {}: {}", output_file, e.to_string());
            Err(e)
        });
    }
    Ok(())
}
