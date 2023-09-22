#[cfg(feature = "howto")]
use std::collections::HashMap;

use anyhow::{bail, Result};
use cocmd::core::sources_manager::SourcesManager;
#[cfg(feature = "howto")]
use levenshtein::levenshtein;
use termimad;

pub fn show_sources(sources_manager: &mut SourcesManager) -> Result<cocmd::CmdExit> {
    let mut table = String::new();
    let skin = termimad::MadSkin::default();

    let sources = sources_manager.sources.values();

    if sources.len() > 0 {
        // Append the markdown table header
        table.push_str("| Source Name | #Aliases | #Automations | #Paths | Path |\n");
        table.push_str("|------------|----------|--------------|--------|-------|\n");

        // Iterate through sources and append rows to the table
        for source in sources {
            let aliases_count = if source.is_legit_cocmd_source() {
                match source.aliases() {
                    Some(aliases_str) => aliases_str.split('\n').count(),
                    _ => 0,
                }
            } else {
                0
            };
            let automations_count = if source.is_legit_cocmd_source() {
                source.automations(&sources_manager.settings).len()
            } else {
                0
            };
            let paths_count = if source.is_legit_cocmd_source() {
                source.paths().len()
            } else {
                0
            };

            table.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                source.uri,
                aliases_count,
                automations_count,
                paths_count,
                source.location().to_str().unwrap()
            ));
        }
    } else {
        table = String::from("No sources yet");
    }

    skin.print_text(&table);

    Ok(cocmd::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}

pub fn show_source(sources_manager: &mut SourcesManager, uri: String) -> Result<cocmd::CmdExit> {
    let source = &sources_manager.sources[&uri];
    let skin = termimad::MadSkin::default();

    if !source.is_legit_cocmd_source() {
        bail!("Source {} is not a legit cocmd source", uri);
    }

    skin.print_text(&format!("# {}", uri));
    skin.print_text(&format!(
        "- location: {}",
        source.location().to_str().unwrap()
    ));

    if let Some(alias) = &source.aliases() {
        skin.print_text("## aliases");
        skin.print_text(&format!("```{}```", alias));
    }

    skin.print_text("## automations");
    // Apply automations as aliases
    for automation in &source.automations(&sources_manager.settings) {
        skin.print_text(&format!("- `{}.{}`", source.name(), automation.name));
    }

    skin.print_text("## paths");

    for p in &source.paths() {
        skin.print_text(&format!("- `{}`", p));
    }

    Ok(cocmd::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}

#[cfg(feature = "howto")]
pub fn howto(sources_manager: &mut SourcesManager, query: String) -> Result<cocmd::CmdExit> {
    // lookup in all descriptions of automation for what matches best to query variable
    // use tokenization and levenshtein distance to find best match

    let mut best_match = String::new();
    let mut best_match_score = usize::MAX;

    // clean query with tfidf algorithm, call it query_clean
    let query_clean = query.clone();

    for (name, a) in sources_manager.automations().iter() {
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
