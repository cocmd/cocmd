use std::path::Path;

use anyhow::{bail, Result};
use cocmd::core::source::Source;
use cocmd::core::sources_manager::SourcesManager;
use cocmd::utils::repository::find_cocmd_files;
use cocmd_package::{get_provider, LOCAL_PROVIDER};
use console::Style;
use dialoguer::Confirm;
use tracing::info;

pub fn install_source(
    sources_manager: &mut SourcesManager,
    source: &str,
    dont_ask: bool,
) -> Result<cocmd::CmdExit> {
    info!("Installing source {:?}", source);

    let settings = &sources_manager.settings;

    let provider = get_provider(&source.to_string(), &settings.runtime_dir).unwrap();
    let localpath = provider.local_path();

    if !provider.is_exists_locally() {
        info!("Source not found locally. Downloading...");
        match provider.download() {
            Ok(downloaded_path) => {
                info!("Downloaded source to {:?}", downloaded_path);
            }
            Err(e) => {
                bail!("Failed to download source: {}", e);
            }
        }
    }

    let locations = if provider.name() == LOCAL_PROVIDER {
        find_cocmd_files(&localpath, sources_manager.settings.scan_depth)
    } else {
        vec![localpath.to_str().unwrap().to_string()]
    };

    let lst_locs = locations.join("\n  - ");
    let style = Style::new().bold().green();
    if provider.name() == LOCAL_PROVIDER {
        println!(
            "found {} cocmd sources in this path:\n  - {}",
            locations.len(),
            lst_locs
        );
    }

    if provider.name() != LOCAL_PROVIDER
        || dont_ask
        || Confirm::new()
            .with_prompt("Do you want to continue?")
            .interact()?
    {
        for loc in locations {
            let source: Source = Source::new(
                if provider.name() == LOCAL_PROVIDER {
                    loc.clone()
                } else {
                    let source_label = source.to_string();
                    source_label.clone()
                },
                &Path::new(&loc).to_path_buf(),
                &sources_manager.settings,
            );
            let uri = source.uri.clone();
            sources_manager.add_source(source);
            println!("{}", style.apply_to(format!("Source '{}' added", uri)));
        }
    } else {
        println!("{}", style.apply_to("Skipped. you answered 'NO'"));
    }

    Ok(cocmd::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}
