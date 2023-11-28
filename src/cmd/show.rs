#[cfg(feature = "howto")]
use std::collections::HashMap;

use anyhow::{bail, Result};
#[cfg(feature = "howto")]
use levenshtein::levenshtein;

use crate::core::packages_manager::PackagesManager;
use crate::output::print_md;

pub fn show_packages(packages_manager: &mut PackagesManager) -> Result<()> {
    let mut table = String::new();

    let packages = packages_manager.packages.values();

    if packages.len() > 0 {
        // Append the markdown table header
        table.push_str("| Package Name | #Aliases | #Automations | #Paths | Path |\n");
        table.push_str("|------------|----------|--------------|--------|-------|\n");

        // Iterate through packages and append rows to the table
        for package in packages {
            if !package.is_legit_cocmd_package() {
                continue;
            }
            table.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                package.name(),
                package.get_aliases_count(),
                package.get_automations_count(&packages_manager.settings),
                package.get_paths_count(),
                package.location().to_str().unwrap()
            ));
        }
    } else {
        table = String::from("No packages yet");
    }

    print_md(&table);

    Ok(())
}

pub fn show_package(packages_manager: &mut PackagesManager, uri: String) -> Result<()> {
    let package = packages_manager
        .get_package(uri.clone())
        .unwrap_or_else(|| panic!("Can't get package {}", &uri));

    if !package.is_legit_cocmd_package() {
        bail!("Package {} is not a legit cocmd package", uri);
    }

    package.print_doc(&packages_manager.settings, true, true, None);

    Ok(())
}

#[cfg(feature = "howto")]
pub fn howto(packages_manager: &mut PackagesManager, query: String) -> Result<()> {
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

    Ok(())
}
