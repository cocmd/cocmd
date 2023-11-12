use std::path::PathBuf;

use anyhow::{anyhow, Result};
use log::{error, info};

use crate::core::packages_manager::PackagesManager;
use crate::package_provider::get_provider;

pub fn uninstall_package(packages_manager: &mut PackagesManager, package_name: &str) -> Result<()> {
    // Retrieve the package
    let package = match packages_manager.get_package(package_name.to_string()) {
        Some(pkg) => pkg,
        None => {
            error!("Package '{}' is not installed.", package_name);
            return Ok(());
        }
    };

    // Use get_provider to determine the provider of the package
    let provider = get_provider(
        &package.uri,
        &packages_manager.settings.runtime_dir,
        Some("tbd".to_string()),
    )
    .map_err(|_| anyhow!("Failed to get provider for package '{}'", package_name))?;

    // Check if the provider is local
    if provider.is_provider_local() {
        info!("Detected a local package uninstall, removing path from package.txt file only.");
        // Proceed with the removal process
        return packages_manager
            .remove_package(package_name)
            .map_err(|e| anyhow!(e));
    }

    // Check if the installation path is within the runtime directory
    let installation_path = provider.get_installation_path(); // Use provider to get the installation path
    let runtime_dir = PathBuf::from(&packages_manager.settings.runtime_dir);
    if !installation_path.starts_with(runtime_dir) {
        error!(
            "Package '{}' is not in the runtime directory and will not be uninstalled.",
            package_name
        );
        return Ok(());
    }

    // If the package is in the runtime directory, proceed with uninstallation
    info!("Uninstalling package {:?}", package_name);

    // Remove the package
    match packages_manager.remove_package(package_name) {
        Ok(_) => {
            info!("Package '{}' was successfully uninstalled.", package_name);
            Ok(())
        }
        Err(e) => Err(anyhow!(e)),
    }
}
