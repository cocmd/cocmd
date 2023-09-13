use anyhow::Result;
use cocmd::core::sources_manager::SourcesManager;
use termimad;

pub fn show_sources(sources_manager: &SourcesManager) -> Result<cocmd::CmdExit> {
    let mut table = String::new();
    let skin = termimad::MadSkin::default();

    let sources = sources_manager.sources.values();

    if sources.len() > 0 {
        // Append the markdown table header
        table.push_str("| Source Name | #Aliases | #Automations | #Paths | Path |\n");
        table.push_str("|------------|----------|--------------|--------|-------|\n");

        // Iterate through sources and append rows to the table
        for source in sources {
            let aliases_count = match source.aliases() {
                Some(aliases_str) => aliases_str.split('\n').count(),
                _ => 0,
            };
            let automations_count = source.automations(&sources_manager.settings).len();
            let paths_count = source.paths().len();

            table.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                source.name(),
                aliases_count,
                automations_count,
                paths_count,
                source.location()
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

pub fn show_source(sources_manager: &SourcesManager, name: String) -> Result<cocmd::CmdExit> {
    let source = &sources_manager.sources[&name];
    let skin = termimad::MadSkin::default();

    skin.print_text(&format!("# {}", name));
    skin.print_text(&format!("- location: {}", source.location()));

    if let Some(alias) = &source.aliases() {
        skin.print_text(&format!("## aliases"));
        skin.print_text(&format!("```{}```", alias));
    }

    skin.print_text(&format!("## automations"));
    // Apply automations as aliases
    for automation in &source.automations(&sources_manager.settings) {
        skin.print_text(&format!("- `{}.{}`", name, automation.name));
    }

    skin.print_text(&format!("## paths"));

    for p in &source.paths() {
        skin.print_text(&format!("- `{}`", p));
    }

    Ok(cocmd::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}
