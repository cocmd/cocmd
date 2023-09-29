use std::path::Path;

use anyhow::{bail, Result};
use cocmd::core::package::Package;
use cocmd::core::packages_manager::PackagesManager;
use cocmd::utils::repository::find_cocmd_files;
use cocmd_package::{get_provider, LOCAL_PROVIDER};
use console::Style;
use dialoguer::Confirm;
use tracing::info;

pub fn install_package(
    packages_manager: &mut PackagesManager,
    package: &str,
    dont_ask: bool,
) -> Result<cocmd::CmdExit> {
    info!("Installing package {:?}", package);

    let settings = &packages_manager.settings;

    let provider = get_provider(&package.to_string(), &settings.runtime_dir).unwrap();
    let localpath = provider.local_path();

    if !provider.is_exists_locally() {
        info!("Package not found locally. Downloading...");
        match provider.download() {
            Ok(downloaded_path) => {
                info!("Downloaded package to {:?}", downloaded_path);
            }
            Err(e) => {
                bail!("Failed to download package: {}", e);
            }
        }
    }

    let locations = if provider.name() == LOCAL_PROVIDER {
        find_cocmd_files(&localpath, packages_manager.settings.scan_depth)
    } else {
        vec![localpath.to_str().unwrap().to_string()]
    };

    let lst_locs = locations.join("\n  - ");
    let style = Style::new().bold().green();
    if provider.name() == LOCAL_PROVIDER {
        println!(
            "found {} cocmd packages in this path:\n  - {}",
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
            let package: Package = Package::new(
                if provider.name() == LOCAL_PROVIDER {
                    loc.clone()
                } else {
                    let package_label = package.to_string();
                    package_label.clone()
                },
                &Path::new(&loc).to_path_buf(),
                &packages_manager.settings,
            );
            let uri = package.uri.clone();
            packages_manager.add_package(package);
            info!("Package '{}' added", uri);
        }
    } else {
        println!("{}", style.apply_to("Skipped. you answered 'NO'"));
    }

    Ok(cocmd::CmdExit {
        code: exitcode::OK,
        message: None,
    })
}
