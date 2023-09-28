use anyhow::Result;
use cocmd::core::sources_manager::SourcesManager;

pub fn run_profile_loader(sources_manager: &mut SourcesManager) -> Result<cocmd::CmdExit> {
    for source in sources_manager.sources.values() {
        if !(source.is_legit_cocmd_source()) {
            println!("# Skipping source {}", &source.uri);
            continue;
        }
        println!("#cocmd aliases for source {}", source.name());

        if let Some(alias) = &source.aliases() {
            println!("{}", alias);
        }

        println!("#cocmd automations for source {}", source.name());

        // Apply automations as aliases
        for automation in &source.automations(&sources_manager.settings, Some(true)) {
            println!(
                r#"alias {}.{}="cocmd run {}.{}""#,
                source.name(),
                automation.name,
                source.name(),
                automation.name
            );
        }

        println!("# cocmd paths for source {}", source.name());

        for p in &source.paths() {
            println!(r#"export PATH="{}:$PATH""#, p);
        }
    }

    Ok(cocmd::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}
