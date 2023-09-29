#[cfg(feature = "howto")]
use std::collections::HashMap;

use anyhow::{bail, Result};
use cocmd::core::packages_manager::PackagesManager;
#[cfg(feature = "howto")]
use levenshtein::levenshtein;
use termimad;

pub fn show_packages(packages_manager: &mut PackagesManager) -> Result<cocmd::CmdExit> {
    let mut table = String::new();
    let skin = termimad::MadSkin::default();

    let packages = packages_manager.packages.values();

    if packages.len() > 0 {
        // Append the markdown table header
        table.push_str("| Package Name | #Aliases | #Automations | #Paths | Path |\n");
        table.push_str("|------------|----------|--------------|--------|-------|\n");

        // Iterate through packages and append rows to the table
        for package in packages {
            let aliases_count = if package.is_legit_cocmd_package() {
                match package.aliases() {
                    Some(aliases_str) => aliases_str.split('\n').count(),
                    _ => 0,
                }
            } else {
                0
            };
            let automations_count = if package.is_legit_cocmd_package() {
                package
                    .automations(&packages_manager.settings, Some(true))
                    .len()
            } else {
                0
            };
            let paths_count = if package.is_legit_cocmd_package() {
                package.paths().len()
            } else {
                0
            };

            table.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                package.uri,
                aliases_count,
                automations_count,
                paths_count,
                package.location().to_str().unwrap()
            ));
        }
    } else {
        table = String::from("No packages yet");
    }

    skin.print_text(&table);

    Ok(cocmd::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}

pub fn show_package(packages_manager: &mut PackagesManager, uri: String) -> Result<cocmd::CmdExit> {
    let package = packages_manager
        .get_package(uri.clone())
        .unwrap_or_else(|| panic!("Can't get package {}", &uri));

    if !package.is_legit_cocmd_package() {
        bail!("Package {} is not a legit cocmd package", uri);
    }

    package.print_doc(&packages_manager.settings, true, true);

    Ok(cocmd::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}

#[cfg(feature = "howto")]
pub fn howto(packages_manager: &mut PackagesManager, query: String) -> Result<cocmd::CmdExit> {
    // lookup in all descriptions of automation for what matches best to query variable
    // use tokenization and levenshtein distance to find best match

    let mut best_match = String::new();
    let mut best_match_score = usize::MAX;

    // clean query with tfidf algorithm, call it query_clean
    let query_clean = query.clone();

    for (name, a) in packages_manager.automations().iter() {
        let score = levenshtein(
            &query,
            a.content.as_ref().unwrap().description.as_ref().unwrap(),
        );
        if score < best_match_score {
            best_match = name.clone();
            best_match_score = score;
        }
    }

    let skin = termimad::MadSkin::default();
    skin.print_text(&best_match);

    Ok(cocmd::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}
