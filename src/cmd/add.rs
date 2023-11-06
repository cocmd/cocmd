use std::path::Path;

use anyhow::{Error, Result};
use console::Style;
use dialoguer::Confirm;
use log::info;

use crate::core::package::Package;
use crate::core::packages_manager::PackagesManager;
use crate::core::utils::repository::find_cocmd_files;
use crate::package_provider::{get_provider, LOCAL_PROVIDER};
pub fn install_package(
    packages_manager: &mut PackagesManager,
    package: &str,
    version: Option<String>,
    dont_ask: bool,
) -> Result<(), Error> {
    info!("Installing package {:?}", package);

    let settings = &packages_manager.settings;

    let provider = get_provider(&package.to_string(), &settings.runtime_dir, version).unwrap();
    let localpath = provider.local_path();

    if !provider.is_exists_locally() {
        info!("Package not found locally. Downloading...");
        match provider.download() {
            Ok(downloaded_path) => {
                info!("Downloaded package to {:?}", downloaded_path);
            }
            Err(e) => {
                return Err(e);
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

    if locations.is_empty() {
        info!("No cocmd packages found in this path");
        return Ok(());
    }

    if provider.name() == LOCAL_PROVIDER {
        info!(
            "found {} cocmd packages in this path:\n  - {}",
            locations.len(),
            lst_locs
        );
    }
    if provider.name() != LOCAL_PROVIDER
        || dont_ask
        || Confirm::new()
            .with_prompt("Do you want to continue?")
            .interact()
            .unwrap()
    {
        for loc in locations {
            let package: Package = Package::new(
                if provider.name() == LOCAL_PROVIDER {
                    loc.clone()
                } else {
                    let package_label = package.to_string();
                    package_label.clone()
                },
                Path::new(&loc),
                &packages_manager.settings,
            );
            let uri = package.uri.clone();
            packages_manager.add_package(package.clone());
            info!("Package '{}' was installed:", uri);
            info!("- ✅ {} aliases available now", package.get_aliases_count());
            info!(
                "- ✅ {} automations available now",
                package.get_automations_count(&packages_manager.settings)
            );
            info!(
                "- ✅ {} paths available now in PATH env",
                package.get_paths_count()
            );
            info!(
                "- run `cocmd show package {}` for more details",
                package.name()
            );
        }
    } else {
        info!("{}", style.apply_to("Skipped. you answered 'NO'"));
    }

    Ok(())
}
