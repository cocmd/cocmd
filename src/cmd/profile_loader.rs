use anyhow::Result;

use super::CmdExit;
use crate::core::packages_manager::PackagesManager;

pub fn run_profile_loader(packages_manager: &mut PackagesManager) -> Result<CmdExit> {
    for package in packages_manager.packages.values() {
        if !(package.is_legit_cocmd_package()) {
            println!("# Skipping package {}", &package.uri);
            continue;
        }
        println!("#cocmd aliases for package {}", package.name());

        if let Some(alias) = &package.aliases() {
            println!("{}", alias);
        }

        println!("#cocmd automations for package {}", package.name());

        // Apply automations as aliases
        for automation in &package.automations(&packages_manager.settings, Some(true)) {
            println!(
                r#"alias {}.{}="cocmd run {}.{}""#,
                package.name(),
                automation.name,
                package.name(),
                automation.name
            );
        }

        println!("# cocmd paths for package {}", package.name());

        for p in &package.paths(true) {
            println!(r#"export PATH="{}:$PATH""#, p);
        }
    }

    Ok(CmdExit {
        code: exitcode::OK,
        message: None,
    })
}
